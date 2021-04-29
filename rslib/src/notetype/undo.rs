// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use crate::prelude::*;

#[derive(Debug)]

pub(crate) enum UndoableNotetypeChange {
    Added(Box<Notetype>),
    Updated(Box<Notetype>),
    Removed(Box<Notetype>),
}

impl Collection {
    pub(crate) fn undo_notetype_change(&mut self, change: UndoableNotetypeChange) -> Result<()> {
        match change {
            UndoableNotetypeChange::Added(nt) => self.remove_notetype_only_undoable(*nt),
            UndoableNotetypeChange::Updated(nt) => {
                let current = self
                    .storage
                    .get_notetype(nt.id)?
                    .ok_or_else(|| AnkiError::invalid_input("notetype disappeared"))?;
                self.update_notetype_undoable(&nt, current)
            }
            UndoableNotetypeChange::Removed(nt) => self.restore_deleted_notetype(*nt),
        }
    }

    pub(crate) fn remove_notetype_only_undoable(&mut self, notetype: Notetype) -> Result<()> {
        self.storage.remove_notetype(notetype.id)?;
        self.save_undo(UndoableNotetypeChange::Removed(Box::new(notetype)));
        Ok(())
    }

    pub(super) fn add_notetype_undoable(
        &mut self,
        notetype: &mut Notetype,
    ) -> Result<(), AnkiError> {
        self.storage.add_notetype(notetype)?;
        self.save_undo(UndoableNotetypeChange::Added(Box::new(notetype.clone())));
        Ok(())
    }

    pub(super) fn update_notetype_undoable(
        &mut self,
        notetype: &Notetype,
        original: Notetype,
    ) -> Result<()> {
        self.save_undo(UndoableNotetypeChange::Updated(Box::new(original)));
        self.storage
            .add_or_update_notetype_with_existing_id(notetype)
    }

    fn restore_deleted_notetype(&mut self, notetype: Notetype) -> Result<()> {
        self.storage
            .add_or_update_notetype_with_existing_id(&notetype)?;
        self.save_undo(UndoableNotetypeChange::Added(Box::new(notetype)));
        Ok(())
    }
}
