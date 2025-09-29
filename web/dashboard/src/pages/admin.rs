use crate::{
    components::common::{Header, TablePlaceholder},
    models::product::{CreateProduct, UpdateProduct},
    models::Product,
    services::admin_service,
    state::AppState,
    Route,
};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use uuid::Uuid;

#[component]
pub fn AdminPage() -> Element {
    let state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let (current_user_opt, token_opt) = {
        let state_read = state.read();
        (state_read.current_user.clone(), state_read.token.clone())
    };

    let user_missing = current_user_opt.is_none();
    let token_missing = token_opt.is_none();

    let nav = navigator.clone();
    use_effect(move || {
        if user_missing || token_missing {
            nav.push(Route::LoginPage {});
        }
    });

    if user_missing || token_missing {
        return rsx!(div { "Redirecting..." });
    }

    let current_user = current_user_opt.unwrap();
    if current_user.role != crate::models::UserRole::Admin {
        return rsx!(div { "Unauthorized" });
    }

    let token = token_opt.unwrap();
    let product_token = token.clone();
    let products = use_resource(move || {
        let token = product_token.clone();
        async move {
            admin_service::get_products(&token)
                .await
                .unwrap_or_default()
        }
    });

    rsx! {
        div {
            class: "main-content",
            Header { current_user: current_user.clone() }
            main {
                class: "page-content",
                div {
                    class: "admin-header",
                    h3 {
                        class: "page-title",
                        svg {
                            class: "page-title-icon",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"
                            }
                        }
                        "Admin Dashboard"
                    }
                    p {
                        class: "page-subtitle",
                        "Manage your clothing products and store settings"
                    }
                }

                div {
                    class: "admin-container",
                    div {
                        class: "admin-left-panel",
                        ProductCreationCard { products: products.clone() }
                    }
                    div {
                        class: "admin-right-panel",
                        ProductListCard { products: products.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn ProductCreationCard(products: Resource<Vec<Product>>) -> Element {
    rsx! {
        div {
            class: "admin-card creation-card",
            div {
                class: "card-header",
                h4 {
                    class: "card-title",
                    svg {
                        class: "card-title-icon",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 6v6m0 0v6m0-6h6m-6 0H6"
                        }
                    }
                    "Create New Product"
                }
                p {
                    class: "card-subtitle",
                    "Add new clothing items to your store"
                }
            }
            ProductCreateForm { products: products.clone() }
        }
    }
}

#[component]
fn ProductListCard(products: Resource<Vec<Product>>) -> Element {
    let mut show_edit_modal = use_signal(|| false);
    let mut product_to_edit = use_signal(|| None::<Product>);

    let handle_edit = move |product: Product| {
        product_to_edit.set(Some(product));
        show_edit_modal.set(true);
    };

    rsx! {
        div {
            class: "admin-card list-card",
            div {
                class: "card-header",
                h4 {
                    class: "card-title",
                    svg {
                        class: "card-title-icon",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                        }
                    }
                    "Product Inventory"
                }
                p {
                    class: "card-subtitle",
                    "Manage your clothing product catalog"
                }
            }
            ProductTable { products: products.clone(), on_edit: handle_edit }
        }
        if show_edit_modal() {
            if let Some(product) = product_to_edit() {
                EditProductModal {
                    show: show_edit_modal,
                    product: product,
                    products: products.clone()
                }
            }
        }
    }
}

#[component]
fn EditProductModal(
    show: Signal<bool>,
    product: Product,
    products: Resource<Vec<Product>>,
) -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let mut name = use_signal(|| product.name.clone());
    let mut description = use_signal(|| product.description.clone());
    let mut price = use_signal(|| product.price);
    let mut image_url = use_signal(|| product.image_url.clone().unwrap_or_default());
    let mut category = use_signal(|| product.category.clone().unwrap_or_default());
    let mut in_stock = use_signal(|| product.in_stock);
    let feedback_message = use_signal(|| None::<String>);

    let handle_submit = move |_| {
        let Some(token) = app_state.read().token.clone() else {
            return;
        };
        let updated_product = UpdateProduct {
            name: Some(name()),
            description: Some(description()),
            price: Some(price()),
            image_url: if image_url().is_empty() {
                None
            } else {
                Some(image_url())
            },
            category: if category().is_empty() {
                None
            } else {
                Some(category())
            },
            in_stock: Some(in_stock()),
        };
        let mut products_resource = products.clone();
        let mut feedback = feedback_message.to_owned();

        spawn(async move {
            match admin_service::update_product(&token, product.id, updated_product).await {
                Ok(_) => {
                    feedback.set(Some("Product updated successfully.".to_string()));
                    products_resource.restart();
                    gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
                    show.set(false);
                }
                Err(_) => feedback.set(Some("Failed to update product.".to_string())),
            }
        });
    };

    rsx! {
        div {
            class: "modal-backdrop",
            div {
                class: "modal",
                div {
                    class: "modal-header",
                    h3 { "Edit Product" }
                    button {
                        class: "modal-close",
                        onclick: move |_| show.set(false),
                        "×"
                    }
                }
                if let Some(message) = feedback_message() {
                    div {
                        class: "feedback-message",
                        "{message}"
                    }
                }
                form {
                    onsubmit: handle_submit,
                    class: "modal-form",
                    div {
                        class: "form-group",
                        label { "Product Name" }
                        input {
                            r#type: "text",
                            value: "{name}",
                            placeholder: "Enter product name",
                            oninput: move |e| name.set(e.value())
                        }
                    }
                    div {
                        class: "form-group",
                        label { "Description" }
                        textarea {
                            value: "{description}",
                            placeholder: "Enter product description",
                            oninput: move |e| description.set(e.value())
                        }
                    }
                    div {
                        class: "form-row",
                        div {
                            class: "form-group",
                            label { "Price ($)" }
                            input {
                                r#type: "number",
                                step: "0.01",
                                value: "{price}",
                                placeholder: "0.00",
                                oninput: move |e| price.set(e.value().parse().unwrap_or(0.0))
                            }
                        }
                        div {
                            class: "form-group",
                            label { "Category" }
                            input {
                                r#type: "text",
                                value: "{category}",
                                placeholder: "e.g., T-Shirts, Jeans, Accessories",
                                oninput: move |e| category.set(e.value())
                            }
                        }
                    }
                    div {
                        class: "form-group",
                        label { "Image URL" }
                        input {
                            r#type: "url",
                            value: "{image_url}",
                            placeholder: "https://example.com/image.jpg",
                            oninput: move |e| image_url.set(e.value())
                        }
                        if !image_url().is_empty() {
                            div {
                                class: "image-preview",
                                img {
                                    src: {
                                        let url = image_url();
                                        if url.starts_with("http") {
                                            url
                                        } else {
                                            format!("http://127.0.0.1:8081{}", url)
                                        }
                                    },
                                    alt: "Preview",
                                    class: "preview-image"
                                }
                            }
                        }
                    }
                    div {
                        class: "form-group checkbox-group",
                        label {
                            class: "checkbox-label",
                            input {
                                r#type: "checkbox",
                                checked: in_stock(),
                                onchange: move |e| in_stock.set(e.value() == "true")
                            }
                            span { "In Stock" }
                        }
                    }
                    div {
                        class: "modal-actions",
                        button {
                            r#type: "button",
                            class: "btn btn-secondary",
                            onclick: move |_| show.set(false),
                            "Cancel"
                        }
                        button {
                            r#type: "submit",
                            class: "btn btn-primary",
                            "Update Product"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProductCreateForm(products: Resource<Vec<Product>>) -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut price = use_signal(|| 0.0);
    let mut image_url = use_signal(|| String::new());
    let mut category = use_signal(|| String::new());
    let mut in_stock = use_signal(|| true);
    let feedback_message = use_signal(|| None::<String>);

    let handle_submit = move |_| {
        let Some(token) = app_state.read().token.clone() else {
            return;
        };
        let new_product = CreateProduct {
            name: name(),
            description: description(),
            price: price(),
            image_url: if image_url().is_empty() {
                None
            } else {
                Some(image_url())
            },
            category: if category().is_empty() {
                None
            } else {
                Some(category())
            },
            in_stock: in_stock(),
        };
        let mut products_resource = products.clone();
        let mut feedback = feedback_message.to_owned();

        spawn(async move {
            match admin_service::create_product(&token, new_product).await {
                Ok(_) => {
                    feedback.set(Some("Product created successfully.".to_string()));
                    products_resource.restart();
                }
                Err(_) => feedback.set(Some("Failed to create product.".to_string())),
            }
        });

        // Clear form
        name.set(String::new());
        description.set(String::new());
        price.set(0.0);
        image_url.set(String::new());
        category.set(String::new());
        in_stock.set(true);
    };

    rsx! {
        div {
            class: "card-content",
            if let Some(message) = feedback_message() {
                div {
                    class: "feedback-message",
                    "{message}"
                }
            }
            form {
                onsubmit: handle_submit,
                class: "product-form",
                div {
                    class: "form-group",
                    label { "Product Name" }
                    input {
                        r#type: "text",
                        value: "{name}",
                        placeholder: "Enter product name",
                        required: true,
                        oninput: move |e| name.set(e.value())
                    }
                }
                div {
                    class: "form-group",
                    label { "Description" }
                    textarea {
                        value: "{description}",
                        placeholder: "Enter product description",
                        required: true,
                        oninput: move |e| description.set(e.value())
                    }
                }
                div {
                    class: "form-row",
                    div {
                        class: "form-group",
                        label { "Price ($)" }
                        input {
                            r#type: "number",
                            step: "0.01",
                            value: "{price}",
                            placeholder: "0.00",
                            required: true,
                            oninput: move |e| price.set(e.value().parse().unwrap_or(0.0))
                        }
                    }
                    div {
                        class: "form-group",
                        label { "Category" }
                        input {
                            r#type: "text",
                            value: "{category}",
                            placeholder: "e.g., T-Shirts, Jeans, Accessories",
                            oninput: move |e| category.set(e.value())
                        }
                    }
                }
                div {
                    class: "form-group",
                    label { "Image URL" }
                    input {
                        r#type: "url",
                        value: "{image_url}",
                        placeholder: "https://example.com/image.jpg",
                        oninput: move |e| image_url.set(e.value())
                    }
                    if !image_url().is_empty() {
                        div {
                            class: "image-preview",
                            img {
                                src: {
                                    let url = image_url();
                                    if url.starts_with("http") {
                                        url
                                    } else {
                                        format!("http://127.0.0.1:8081{}", url)
                                    }
                                },
                                alt: "Preview",
                                class: "preview-image"
                            }
                        }
                    }
                }
                div {
                    class: "form-group checkbox-group",
                    label {
                        class: "checkbox-label",
                        input {
                            r#type: "checkbox",
                            checked: in_stock(),
                            onchange: move |e| in_stock.set(e.value() == "true")
                        }
                        span { "In Stock" }
                    }
                }
                button {
                    r#type: "submit",
                    class: "btn btn-primary btn-full",
                    "Create Product"
                }
            }
        }
    }
}

#[component]
fn ProductTable(products: Resource<Vec<Product>>, on_edit: EventHandler<Product>) -> Element {
    let app_state = use_context::<Signal<AppState>>();

    let handle_delete = move |product_id: Uuid| {
        let Some(token) = app_state.read().token.clone() else {
            return;
        };
        let mut products_resource = products.clone();
        spawn(async move {
            let _ = admin_service::delete_product(&token, product_id).await;
            products_resource.restart();
        });
    };

    rsx! {
        div {
            class: "table-container",
            table {
                class: "product-table",
                thead {
                    tr {
                        th { "Image" }
                        th { "Name" }
                        th { "Price" }
                        th { "Category" }
                        th { "Stock" }
                        th { "Actions" }
                    }
                }
                tbody {
                    if let Some(product_list) = products.read().as_ref() {
                        if product_list.is_empty() {
                            tr {
                                td {
                                    colspan: 6,
                                    class: "empty-state",
                                    div {
                                        class: "empty-state-content",
                                        svg {
                                            class: "empty-state-icon",
                                            fill: "none",
                                            stroke: "currentColor",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                stroke_width: "2",
                                                d: "M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"
                                            }
                                        }
                                        p { "No products found. Create your first product above." }
                                    }
                                }
                            }
                        } else {
                            for product in product_list {
                                ProductRow {
                                    product: product.clone(),
                                    on_edit: on_edit,
                                    on_delete: handle_delete
                                }
                            }
                        }
                    } else {
                        TablePlaceholder { rows: 5, cols: 6 }
                    }
                }
            }
        }
    }
}

#[component]
fn ProductRow(
    product: Product,
    on_edit: EventHandler<Product>,
    on_delete: EventHandler<Uuid>,
) -> Element {
    let price_display = format!("${:.2}", product.price);
    let category_display = product.category.as_deref().unwrap_or("Uncategorized");
    let stock_status = if product.in_stock {
        "In Stock"
    } else {
        "Out of Stock"
    };
    let stock_class = if product.in_stock {
        "stock-in"
    } else {
        "stock-out"
    };
    let product_id = product.id;
    let product_for_edit = product.clone();

    rsx! {
        tr {
            class: "product-row",
            td {
                class: "product-image-cell",
                if let Some(image_url) = &product.image_url {
                    img {
                        src: {
                            if image_url.starts_with("http") {
                                image_url.clone()
                            } else {
                                format!("http://127.0.0.1:8081{}", image_url)
                            }
                        },
                        alt: "{product.name}",
                        class: "product-thumbnail"
                    }
                } else {
                    div {
                        class: "no-image-placeholder",
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            width: "24",
                            height: "24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                            }
                        }
                    }
                }
            }
            td {
                class: "product-name-cell",
                div {
                    class: "product-info",
                    div {
                        class: "product-name",
                        "{product.name}"
                    }
                    if !product.description.is_empty() {
                        div {
                            class: "product-description",
                            "{product.description}"
                        }
                    }
                }
            }
            td {
                class: "price-cell",
                "{price_display}"
            }
            td {
                class: "category-cell",
                "{category_display}"
            }
            td {
                span {
                    class: "stock-status {stock_class}",
                    "{stock_status}"
                }
            }
            td {
                class: "actions-cell",
                div {
                    class: "action-buttons",
                    button {
                        class: "btn btn-edit btn-sm",
                        onclick: move |_| on_edit.call(product_for_edit.clone()),
                        title: "Edit product",
                        "Edit"
                    }
                    button {
                        class: "btn btn-delete btn-sm",
                        onclick: move |_| on_delete.call(product_id),
                        title: "Delete product",
                        "Delete"
                    }
                }
            }
        }
    }
}
