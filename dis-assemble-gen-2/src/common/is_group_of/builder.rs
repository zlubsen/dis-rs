use crate::enumerations::IsGroupOfGroupedEntityCategory;
use crate::is_group_of::model::{GroupEntityDescription, GroupReferencePoint, IsGroupOf};
use crate::model::EntityId;

pub struct IsGroupOfBuilder(IsGroupOf);

impl Default for IsGroupOfBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IsGroupOfBuilder {
    #[must_use]
    pub fn new() -> Self {
        IsGroupOfBuilder(IsGroupOf::default())
    }

    #[must_use]
    pub fn new_from_body(body: IsGroupOf) -> Self {
        IsGroupOfBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> IsGroupOf {
        self.0
    }

    #[must_use]
    pub fn with_group_id(mut self, group_id: EntityId) -> Self {
        self.0.group_id = group_id;
        self
    }

    #[must_use]
    pub fn with_grouped_entity_category(
        mut self,
        grouped_entity_category: IsGroupOfGroupedEntityCategory,
    ) -> Self {
        self.0.grouped_entity_category = grouped_entity_category;
        self
    }

    #[must_use]
    pub fn with_group_reference_point(
        mut self,
        group_reference_point: GroupReferencePoint,
    ) -> Self {
        self.0.group_reference_point = group_reference_point;
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: GroupEntityDescription) -> Self {
        self.0.descriptions.push(description);
        self
    }

    #[must_use]
    pub fn with_descriptions(mut self, descriptions: Vec<GroupEntityDescription>) -> Self {
        self.0.descriptions = descriptions;
        self
    }
}
