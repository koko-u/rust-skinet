use crate::shared::params;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::IntoParams)]
#[serde(rename_all = "kebab-case")]
#[into_params(parameter_in = Query)]
pub struct PagingParam {
    #[garde(custom(validators::positive_u32))]
    #[param(required = false, value_type = u32, minimum = 1)]
    pub current_page: Option<String>,
    #[garde(custom(validators::positive_u32))]
    #[param(required = false, value_type = u32, minimum = 1)]
    pub page_size: Option<String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ValidPagingParam {
    pub current_page: params::CurrentPage,
    pub page_size: params::PageSize,
}

impl PagingParam {
    pub fn validate_into(self) -> Result<ValidPagingParam, garde::Report> {
        use garde::Validate as _;
        self.validate()?;

        let current_page = self
            .current_page
            .map(|current_page| {
                if current_page.is_empty() {
                    params::CurrentPage::default()
                } else {
                    serde_plain::from_str::<params::CurrentPage>(&current_page)
                        .expect("current page should be u32")
                }
            })
            .unwrap_or_default();
        let page_size = self
            .page_size
            .map(|page_size| {
                if page_size.is_empty() {
                    params::PageSize::default()
                } else {
                    serde_plain::from_str::<params::PageSize>(&page_size).expect("page size should be u32")
                }
            })
            .unwrap_or_default();

        Ok(ValidPagingParam {
            current_page,
            page_size,
        })
    }
}

impl ValidPagingParam {
    pub fn limit(self) -> i64 {
        self.page_size.into_inner() as i64
    }

    pub fn offset(self) -> i64 {
        let offset = (self.current_page.into_inner() - 1) * self.page_size.into_inner();
        offset as i64
    }
}

mod validators;
