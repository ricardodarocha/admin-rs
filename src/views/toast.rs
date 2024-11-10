use std::sync::Arc;
use minijinja::{Environment, context};
use crate::infra::toast::Toast;

pub fn render_toast(
    env: &Arc<Environment<'static>>, 
    context: Toast
) -> String {

    let tmpl = env.get_template("components/ajaxToast.html").unwrap();

    let class = match context.tipo {        
        crate::infra::toast::TipoToast::Success => "toast-success",
        crate::infra::toast::TipoToast::Info => "toast-info",
        crate::infra::toast::TipoToast::Warn => "toast-warning",
        crate::infra::toast::TipoToast::Error => "toast-error",
    };

    let rendered = tmpl.render(context! {
        toast_icon => context.icon,
        toast_class => class,
        toast_text => context.text,
    }).unwrap();

   rendered
}