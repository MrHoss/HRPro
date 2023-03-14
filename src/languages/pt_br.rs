use std::collections::HashMap;

pub fn pt_br() -> HashMap<&'static str, &'static str> {
    let translations = [
        ("not_found","Nada encontrado"),
        ("AppName", "MHPro"),
        ("register", "Registro"),
        ("new_order", "Novo Pedido +"),
        ("cash_flow", "Fluxo de Caixa"),
        ("bills_to_pay", "Contas a Pagar"),
        ("bills_to_receive", "Contas a Receber"),
        ("dashboard", "Dashboard"),
        ("reports", "Relatórios"),
        ("users", "Usuários"),
        ("integrations", "Integrações"),
        ("company","Empresa"),
        ("settings","Configurações"),
        ("customers","Clientes"),
        ("suppliers","Fornecedores"),
        ("sales","Vendas"),
        ("services","Serviços"),
        ("stock","Estoque"),

        // Adicione mais traduções aqui
    ]
    .iter()
    .cloned()
    .collect();

    translations
}