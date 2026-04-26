use bevy::prelude::*;
use std::vec::Drain;

#[derive(Resource)]
pub struct SelectedCardImage(pub Handle<Image>);

pub struct TableSlotEntity {
    id: Entity,
    vacant: bool,
}

impl TableSlotEntity {
    pub fn new(id: Entity) -> Self {
        Self { id, vacant: true }
    }

    pub fn id(&self) -> Entity {
        self.id
    }

    pub fn is_vacant(&self) -> bool {
        self.vacant
    }

    pub fn occupy(&mut self) {
        self.vacant = false;
    }

    pub fn free(&mut self) {
        self.vacant = true;
    }
}

#[derive(Resource)]
pub struct TableSlots {
    pub slots: Vec<TableSlotEntity>,
}

impl TableSlots {
    pub fn new(slots: Vec<TableSlotEntity>) -> Self {
        Self { slots }
    }

    pub fn add(&mut self, entity: Entity) {
        self.slots.push(TableSlotEntity::new(entity));
    }

    pub fn free_slot(&mut self) -> Option<Entity> {
        for i in 0..self.slots.len() {
            if self.slots[i].is_vacant() {
                self.slots[i].occupy();
                return Some(self.slots[i].id());
            }
        }
        None
    }
}

impl<'a> IntoIterator for &'a mut TableSlots {
    type Item = &'a mut TableSlotEntity;
    type IntoIter = std::slice::IterMut<'a, TableSlotEntity>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.iter_mut()
    }
}

#[derive(Resource)]
pub struct DragCursor {
    entity: Entity,
    to_drag: Option<Entity>,
}

impl DragCursor {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            to_drag: None,
        }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn drag_target(&mut self, drag_target: Entity) {
        self.to_drag = Some(drag_target);
    }

    pub fn take_dragged(&mut self) -> Option<Entity> {
        self.to_drag.take()
    }
}
