use pgrx::pg_sys::Item;

use crate::util::{table_slot::TableSlot, HeapPointer, IndexPointer};

use super::{
    stats::{StatsDistanceComparison, StatsNodeRead},
    storage::{NodeFullDistanceMeasure, Storage, StorageFullDistanceFromHeap},
};

pub struct HeapFullDistanceMeasure<'a, S: Storage + StorageFullDistanceFromHeap> {
    table_slot: Option<TableSlot>,
    storage: &'a S,
}

impl<'a, S: Storage + StorageFullDistanceFromHeap> HeapFullDistanceMeasure<'a, S> {
    pub unsafe fn new<T: StatsNodeRead>(
        storage: &'a S,
        index_pointer: IndexPointer,
        stats: &mut T,
    ) -> Self {
        let slot = storage.get_heap_table_slot_from_index_pointer(index_pointer, stats);
        Self {
            table_slot: Some(slot),
            storage: storage,
        }
    }
}

impl<'a, S: Storage + StorageFullDistanceFromHeap> NodeFullDistanceMeasure
    for HeapFullDistanceMeasure<'a, S>
{
    unsafe fn get_distance<T: StatsNodeRead + StatsDistanceComparison>(
        &self,
        index_pointer: IndexPointer,
        stats: &mut T,
    ) -> f32 {
        let slot = self
            .storage
            .get_heap_table_slot_from_index_pointer(index_pointer, stats);
        stats.record_full_distance_comparison();
        let slice1 = slot.get_pg_vector();
        let slice2 = self.table_slot.as_ref().unwrap().get_pg_vector();
        (self.storage.get_distance_function())(slice1.to_slice(), slice2.to_slice())
    }
}

pub unsafe fn calculate_full_distance<
    S: Storage + StorageFullDistanceFromHeap,
    T: StatsNodeRead + StatsDistanceComparison,
>(
    storage: &S,
    heap_pointer: HeapPointer,
    query: &[f32],
    stats: &mut T,
) -> f32 {
    let slot = storage.get_heap_table_slot_from_heap_pointer(heap_pointer, stats);
    let slice = unsafe { slot.get_pg_vector() };

    stats.record_full_distance_comparison();
    let dist = (storage.get_distance_function())(slice.to_slice(), query);
    debug_assert!(!dist.is_nan());
    dist
}