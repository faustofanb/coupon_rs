use crate::storage::dao::Table;
use crate::storage::entity::template_entity::TemplateDO;
use crate::transfer::request::template_req::TemplateSaveReqDto;

impl Table<'_, TemplateDO> {
    pub async fn add_template(&self, req: &TemplateSaveReqDto) -> Result<(), sqlx::Error> {
        Ok(())
    }
}
