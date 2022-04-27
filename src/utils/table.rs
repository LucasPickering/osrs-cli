use comfy_table::{Cell, CellAlignment, Table};

/// Additional functionality for the [Table](comfy_table::Table) type
pub trait TableExt {
    /// Set the headers of a table in conjunction with an alignment for each
    /// column
    fn set_aligned_header<T: Into<Cell>>(
        &mut self,
        headers: impl IntoIterator<Item = (T, CellAlignment)>,
    ) -> &mut Self;
}

impl TableExt for Table {
    fn set_aligned_header<T: Into<Cell>>(
        &mut self,
        headers: impl IntoIterator<Item = (T, CellAlignment)>,
    ) -> &mut Self {
        let (header, alignments): (Vec<_>, Vec<_>) =
            headers.into_iter().unzip();
        self.set_header(header);
        for (i, alignment) in alignments.into_iter().enumerate() {
            let column =
                self.get_column_mut(i).expect("No column with index {i}");
            column.set_cell_alignment(alignment);
        }
        self
    }
}
