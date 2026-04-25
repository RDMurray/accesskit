// Copyright 2026 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit_atspi_common::PlatformNode;
use zbus::{fdo, interface, names::OwnedUniqueName};

use crate::atspi::{ObjectId, OwnedObjectAddress};

pub(crate) struct TableInterface {
    bus_name: OwnedUniqueName,
    node: PlatformNode,
}

impl TableInterface {
    pub fn new(bus_name: OwnedUniqueName, node: PlatformNode) -> Self {
        Self { bus_name, node }
    }

    fn map_error(&self) -> impl '_ + FnOnce(accesskit_atspi_common::Error) -> fdo::Error {
        |error| crate::util::map_error_from_node(&self.node, error)
    }

    fn node_address(&self, node: Option<accesskit_atspi_common::NodeId>) -> OwnedObjectAddress {
        node.map(|node| {
            ObjectId::Node {
                adapter: self.node.adapter_id(),
                node,
            }
            .to_address(self.bus_name.inner())
        })
        .unwrap_or_else(OwnedObjectAddress::null)
    }
}

#[interface(name = "org.a11y.atspi.Table")]
impl TableInterface {
    #[zbus(property, name = "NRows")]
    fn nrows(&self) -> fdo::Result<i32> {
        self.node.table_n_rows().map_err(self.map_error())
    }

    #[zbus(property, name = "NColumns")]
    fn ncolumns(&self) -> fdo::Result<i32> {
        self.node.table_n_columns().map_err(self.map_error())
    }

    #[zbus(property)]
    fn caption(&self) -> OwnedObjectAddress {
        OwnedObjectAddress::null()
    }

    #[zbus(property)]
    fn summary(&self) -> OwnedObjectAddress {
        OwnedObjectAddress::null()
    }

    #[zbus(property, name = "NSelectedRows")]
    fn nselected_rows(&self) -> fdo::Result<i32> {
        self.node
            .table_selected_rows()
            .and_then(|rows| {
                rows.len()
                    .try_into()
                    .map_err(|_| accesskit_atspi_common::Error::IndexOutOfRange)
            })
            .map_err(self.map_error())
    }

    #[zbus(property, name = "NSelectedColumns")]
    fn nselected_columns(&self) -> i32 {
        0
    }

    fn get_accessible_at(&self, row: i32, column: i32) -> fdo::Result<OwnedObjectAddress> {
        let node = self
            .node
            .table_accessible_at(row, column)
            .map_err(self.map_error())?;
        Ok(self.node_address(node))
    }

    fn get_index_at(&self, row: i32, column: i32) -> fdo::Result<i32> {
        self.node
            .table_index_at(row, column)
            .map_err(self.map_error())
    }

    fn get_row_at_index(&self, index: i32) -> fdo::Result<i32> {
        self.node
            .table_row_at_index(index)
            .map_err(self.map_error())
    }

    fn get_column_at_index(&self, index: i32) -> fdo::Result<i32> {
        self.node
            .table_column_at_index(index)
            .map_err(self.map_error())
    }

    fn get_row_description(&self, row: i32) -> fdo::Result<String> {
        self.node
            .table_row_description(row)
            .map_err(self.map_error())
    }

    fn get_column_description(&self, column: i32) -> fdo::Result<String> {
        self.node
            .table_column_description(column)
            .map_err(self.map_error())
    }

    fn get_row_extent_at(&self, row: i32, column: i32) -> fdo::Result<i32> {
        Ok(self
            .node
            .table_row_column_extents_at_index(
                self.node
                    .table_index_at(row, column)
                    .map_err(self.map_error())?,
            )
            .map_err(self.map_error())?
            .3)
    }

    fn get_column_extent_at(&self, row: i32, column: i32) -> fdo::Result<i32> {
        Ok(self
            .node
            .table_row_column_extents_at_index(
                self.node
                    .table_index_at(row, column)
                    .map_err(self.map_error())?,
            )
            .map_err(self.map_error())?
            .4)
    }

    fn get_row_header(&self, row: i32) -> fdo::Result<OwnedObjectAddress> {
        let node = self.node.table_row_header(row).map_err(self.map_error())?;
        Ok(self.node_address(node))
    }

    fn get_column_header(&self, column: i32) -> fdo::Result<OwnedObjectAddress> {
        let node = self
            .node
            .table_column_header(column)
            .map_err(self.map_error())?;
        Ok(self.node_address(node))
    }

    fn get_selected_rows(&self) -> fdo::Result<Vec<i32>> {
        self.node.table_selected_rows().map_err(self.map_error())
    }

    fn get_selected_columns(&self) -> Vec<i32> {
        Vec::new()
    }

    fn is_row_selected(&self, row: i32) -> fdo::Result<bool> {
        self.node
            .table_is_row_selected(row)
            .map_err(self.map_error())
    }

    fn is_column_selected(&self, _column: i32) -> bool {
        false
    }

    fn is_selected(&self, row: i32, column: i32) -> fdo::Result<bool> {
        self.node
            .table_is_selected(row, column)
            .map_err(self.map_error())
    }

    fn add_row_selection(&self, _row: i32) -> bool {
        false
    }

    fn add_column_selection(&self, _column: i32) -> bool {
        false
    }

    fn remove_row_selection(&self, _row: i32) -> bool {
        false
    }

    fn remove_column_selection(&self, _column: i32) -> bool {
        false
    }

    fn get_row_column_extents_at_index(
        &self,
        index: i32,
    ) -> fdo::Result<(bool, i32, i32, i32, i32, bool)> {
        self.node
            .table_row_column_extents_at_index(index)
            .map_err(self.map_error())
    }
}
