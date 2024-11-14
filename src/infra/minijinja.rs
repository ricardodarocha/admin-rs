
use std::sync::Arc;
use minijinja::Environment;
use crate::helpers;

pub async fn configure_minijinja() -> Arc<Environment<'static>> {
    let mut env = Environment::new();
    env.add_function("url", helpers::url);
    env.add_function("mascara", helpers::mascara);
    env.add_function("anonimizar", helpers::anonimizar);
    env.add_function("fmt_decimal", helpers::fmt_decimal);
    env.add_function("fmt_cpf", helpers::fmt_cpf);
    env.add_function("fmt_cnpj", helpers::fmt_cnpj);
    env.add_function("fmt_cep", helpers::fmt_cep);
    env.add_function("numero_por_extenso", helpers::por_extenso::numero_por_extenso);
    env.add_function("multimidia", helpers::img_src::multimidia);


    env.add_filter("fmtdate", helpers::filter::fmtdate);    
    env.add_filter("fmtdateopt", helpers::filter::fmtdateopt);
    env.add_filter("fmttime", helpers::filter::fmttime);
    env.add_filter("fmttimeopt", helpers::filter::fmttimeopt);
    env.add_filter("fmt", helpers::filter::fmt);
    env.add_filter("fmt3", helpers::filter::fmt3);

    env.add_filter("mascara", helpers::mascara);
    env.add_filter("anonimizar", helpers::anonimizar);
    env.add_filter("fmt_decimal", helpers::fmt_decimal);
    env.add_filter("fmt_cpf", helpers::fmt_cpf);
    env.add_filter("fmt_cnpj", helpers::fmt_cnpj);
    env.add_filter("fmt_cep", helpers::fmt_cep);
    env.add_filter("numero_por_extenso", helpers::por_extenso::numero_por_extenso);

    env.add_function("format", helpers::filter::format_filter);
    // env.add_function("url_for", |route: String| minijinja_utils::url_for(&route));

    env.set_loader(minijinja::path_loader("resources/views"));
    Arc::new(env)
}