use std::collections::HashMap;
use stq_types::*;
use types::*;

use geo::Point as GeoPoint;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: WarehouseId,
    pub store_id: StoreId,
    pub slug: WarehouseSlug,
    pub name: Option<String>,
    pub location: Option<GeoPoint<f64>>,
    pub administrative_area_level_1: Option<String>,
    pub administrative_area_level_2: Option<String>,
    pub country: Option<String>,
    pub locality: Option<String>,
    pub political: Option<String>,
    pub postal_code: Option<String>,
    pub route: Option<String>,
    pub street_number: Option<String>,
    pub address: Option<String>,
    pub place_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarehouseInput {
    #[serde(default = "WarehouseId::new")]
    pub id: WarehouseId,
    pub store_id: StoreId,
    pub name: Option<String>,
    pub location: Option<GeoPoint<f64>>,
    pub administrative_area_level_1: Option<String>,
    pub administrative_area_level_2: Option<String>,
    pub country: Option<String>,
    pub locality: Option<String>,
    pub political: Option<String>,
    pub postal_code: Option<String>,
    pub route: Option<String>,
    pub street_number: Option<String>,
    pub address: Option<String>,
    pub place_id: Option<String>,
}

impl WarehouseInput {
    pub fn new(store_id: StoreId) -> Self {
        Self {
            store_id,
            id: WarehouseId::new(),
            name: Default::default(),
            location: Default::default(),
            administrative_area_level_1: Default::default(),
            administrative_area_level_2: Default::default(),
            country: Default::default(),
            locality: Default::default(),
            political: Default::default(),
            postal_code: Default::default(),
            route: Default::default(),
            street_number: Default::default(),
            address: Default::default(),
            place_id: Default::default(),
        }
    }

    pub fn split_slug(v: Warehouse) -> (WarehouseInput, WarehouseSlug) {
        (
            WarehouseInput {
                id: v.id,
                store_id: v.store_id,
                name: v.name,
                location: v.location,
                administrative_area_level_1: v.administrative_area_level_1,
                administrative_area_level_2: v.administrative_area_level_2,
                country: v.country,
                locality: v.locality,
                political: v.political,
                postal_code: v.postal_code,
                route: v.route,
                street_number: v.street_number,
                address: v.address,
                place_id: v.place_id,
            },
            v.slug,
        )
    }

    pub fn with_slug(self, slug: WarehouseSlug) -> Warehouse {
        Warehouse {
            id: self.id,
            store_id: self.store_id,
            slug,
            name: self.name,
            location: self.location,
            administrative_area_level_1: self.administrative_area_level_1,
            administrative_area_level_2: self.administrative_area_level_2,
            country: self.country,
            locality: self.locality,
            political: self.political,
            postal_code: self.postal_code,
            route: self.route,
            street_number: self.street_number,
            address: self.address,
            place_id: self.place_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stock {
    pub id: StockId,
    pub warehouse_id: WarehouseId,
    pub product_id: ProductId,
    pub quantity: Quantity,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StockMeta {
    pub quantity: Quantity,
}

impl From<Stock> for (ProductId, StockMeta) {
    fn from(v: Stock) -> (ProductId, StockMeta) {
        (
            v.product_id,
            StockMeta {
                quantity: v.quantity,
            },
        )
    }
}

impl From<Stock> for (StockId, WarehouseId, ProductId, StockMeta) {
    fn from(v: Stock) -> Self {
        (
            v.id,
            v.warehouse_id,
            v.product_id,
            StockMeta {
                quantity: v.quantity,
            },
        )
    }
}

pub type StockMap = HashMap<ProductId, StockMeta>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WarehouseUpdateData {
    pub slug: Option<ValueContainer<WarehouseSlug>>,
    pub name: Option<ValueContainer<Option<String>>>,
    pub location: Option<ValueContainer<Option<GeoPoint<f64>>>>,
    pub administrative_area_level_1: Option<ValueContainer<Option<String>>>,
    pub administrative_area_level_2: Option<ValueContainer<Option<String>>>,
    pub country: Option<ValueContainer<Option<String>>>,
    pub locality: Option<ValueContainer<Option<String>>>,
    pub political: Option<ValueContainer<Option<String>>>,
    pub postal_code: Option<ValueContainer<Option<String>>>,
    pub route: Option<ValueContainer<Option<String>>>,
    pub street_number: Option<ValueContainer<Option<String>>>,
    pub address: Option<ValueContainer<Option<String>>>,
    pub place_id: Option<ValueContainer<Option<String>>>,
}

pub trait WarehouseClient {
    fn create_warehouse(&self, new_warehouse: WarehouseInput) -> ApiFuture<Warehouse>;
    fn get_warehouse(&self, warehouse_id: WarehouseIdentifier) -> ApiFuture<Option<Warehouse>>;
    fn update_warehouse(
        &self,
        warehouse_id: WarehouseIdentifier,
        update_data: WarehouseUpdateData,
    ) -> ApiFuture<Option<Warehouse>>;
    fn delete_warehouse(&self, warehouse_id: WarehouseIdentifier) -> ApiFuture<Option<Warehouse>>;
    fn delete_all_warehouses(&self) -> ApiFuture<Vec<Warehouse>>;
    fn get_warehouses_for_store(&self, store_id: StoreId) -> ApiFuture<Vec<Warehouse>>;

    fn set_product_in_warehouse(
        &self,
        warehouse_id: WarehouseId,
        product_id: ProductId,
        quantity: Quantity,
    ) -> ApiFuture<Stock>;
    fn get_product_in_warehouse(
        &self,
        warehouse_id: WarehouseId,
        product_id: ProductId,
    ) -> ApiFuture<Option<Stock>>;
    fn list_products_in_warehouse(&self, warehouse_id: WarehouseId) -> ApiFuture<StockMap>;

    fn get_warehouse_product(&self, warehouse_product_id: StockId) -> ApiFuture<Option<Stock>>;

    /// Find all products with id in all warehouses
    fn find_by_product_id(&self, product_id: ProductId) -> ApiFuture<Vec<Stock>>;
}
