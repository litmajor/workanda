
use sqlx::PgPool;
use crate::models::contract_template::*;
use crate::api::error::ApiError;
use std::collections::HashMap;

pub struct ContractTemplateService {
    pool: PgPool,
}

impl ContractTemplateService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_template(
        &self,
        user_id: i32,
        request: CreateTemplateRequest,
    ) -> Result<ContractTemplate, ApiError> {
        let template = sqlx::query_as!(
            ContractTemplate,
            r#"
            INSERT INTO contract_templates (name, description, category, content, variables, is_public, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            RETURNING id, name, description, category, content, variables, is_public, created_by, created_at, updated_at
            "#,
            request.name,
            request.description,
            request.category,
            request.content,
            &request.variables,
            request.is_public,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(template)
    }

    pub async fn get_templates(
        &self,
        category: Option<String>,
    ) -> Result<Vec<ContractTemplate>, ApiError> {
        let templates = if let Some(cat) = category {
            sqlx::query_as!(
                ContractTemplate,
                "SELECT * FROM contract_templates WHERE category = $1 AND is_public = true ORDER BY created_at DESC",
                cat
            )
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as!(
                ContractTemplate,
                "SELECT * FROM contract_templates WHERE is_public = true ORDER BY created_at DESC"
            )
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(templates)
    }

    pub async fn generate_contract(
        &self,
        request: GenerateContractRequest,
    ) -> Result<GeneratedContract, ApiError> {
        let template = sqlx::query_as!(
            ContractTemplate,
            "SELECT * FROM contract_templates WHERE id = $1",
            request.template_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiError::NotFound("Template not found".to_string()))?;

        let mut content = template.content.clone();
        for (key, value) in request.variable_values.iter() {
            let placeholder = format!("{{{{{}}}}}", key);
            content = content.replace(&placeholder, value);
        }

        Ok(GeneratedContract {
            content,
            contract_id: None,
        })
    }

    pub async fn get_default_templates(&self) -> Vec<ContractTemplate> {
        vec![
            self.create_default_template("Web Development", "Standard web development contract for building websites and web applications"),
            self.create_default_template("Mobile App Development", "Contract for iOS/Android mobile application development"),
            self.create_default_template("Design Services", "UI/UX and graphic design services contract"),
            self.create_default_template("Consulting", "Professional consulting services agreement"),
        ]
    }

    fn create_default_template(&self, category: &str, description: &str) -> ContractTemplate {
        ContractTemplate {
            id: 0,
            name: format!("{} Contract", category),
            description: description.to_string(),
            category: category.to_string(),
            content: self.get_template_content(category),
            variables: vec!["project_name".to_string(), "budget".to_string(), "deadline".to_string()],
            is_public: true,
            created_by: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn get_template_content(&self, category: &str) -> String {
        format!(
            r#"
SERVICE AGREEMENT

This agreement is made on {{{{date}}}} between:
Client: {{{{client_name}}}}
Freelancer: {{{{freelancer_name}}}}

PROJECT DETAILS:
Project Name: {{{{project_name}}}}
Description: {{{{project_description}}}}
Budget: ${{{{budget}}}}
Deadline: {{{{deadline}}}}

SCOPE OF WORK:
{{{{scope_of_work}}}}

PAYMENT TERMS:
- Total project value: ${{{{budget}}}}
- Payment schedule: {{{{payment_schedule}}}}
- Payment method: Escrow via Workanda platform

DELIVERABLES:
{{{{deliverables}}}}

TIMELINE:
Start Date: {{{{start_date}}}}
End Date: {{{{deadline}}}}

INTELLECTUAL PROPERTY:
All work created under this contract will be transferred to the Client upon full payment.

TERMINATION:
Either party may terminate this agreement with {{{{notice_period}}}} days written notice.

SIGNATURES:
Client: ___________________ Date: ___________
Freelancer: ___________________ Date: ___________
            "#
        )
    }
}
