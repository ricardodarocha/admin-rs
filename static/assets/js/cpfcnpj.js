$(".cpf-cnpj").focus(removeMascara);
function removeMascara() {
    $(".cpf-cnpj").unmask();
    $(this).attr("maxlength", "14");
};

// Limpa resultado ao dar focusout em campo
$(".cpf-cnpj").focusout(function () {
    $(".resultadoValidacao").html("");
});

// Permite somente uso de caracteres numéricos
$(".cpf-cnpj").keypress(function (e) {
    if (e.which != 8 && e.which != 0 && (e.which < 48 || e.which > 57)) {
        return false;
    }
});

// Não permite colar caracteres
$('.cpf-cnpj').bind("paste", function (e) {
    e.preventDefault();
});

// Identifica e valida se o campo é de CPF ou CNPJ
$(".executaValidacao").click(CPFouCNPJ);
function CPFouCNPJ() {
    var contador = $(".cpf-cnpj").val().replace(/[^0-9]/g, "").length;
    if (contador == 0) {
        $(".resultadoValidacao").removeClass('success').addClass('alert');
        $(".resultadoValidacao").html("Digite o CPF ou CNPJ");
    } else if (contador == 11) {
        if (validaCPF($(".cpf-cnpj").val())) {
            $(".resultadoValidacao").removeClass('alert').addClass('success');
            $(".resultadoValidacao").html("CPF válido");
        } else {
            $(".resultadoValidacao").removeClass('success').addClass('alert');
            $(".resultadoValidacao").html("CPF inválido");
        }
        $(".cpf-cnpj").mask("999.999.999-99");
    } else if (contador == 14) {
        if (validaCNPJ($(".cpf-cnpj").val())) {
            $(".resultadoValidacao").removeClass('alert').addClass('success');
            $(".resultadoValidacao").html("CNPJ válido");
        } else {
            $(".resultadoValidacao").removeClass('success').addClass('alert');
            $(".resultadoValidacao").html("CNPJ inválido");
        }
        $(".cpf-cnpj").mask("99.999.999/9999-99");
    } else {
        $(".resultadoValidacao").removeClass('success').addClass('alert');
        $(".resultadoValidacao").html("CPF ou CNPJ inválidos");
    }
}

// Valida CPF
function validaCPF(cpf) {
    cpf = cpf.replace(/[^\d]+/g, '');
    if (cpf == '') return false;
    if (
        cpf.length != 11 ||
        cpf == "00000000000" ||
        cpf == "11111111111" ||
        cpf == "22222222222" ||
        cpf == "33333333333" ||
        cpf == "44444444444" ||
        cpf == "55555555555" ||
        cpf == "66666666666" ||
        cpf == "77777777777" ||
        cpf == "88888888888" ||
        cpf == "99999999999" ||
        cpf == "01234567890")
        return false;
    add = 0;

    for (i = 0; i < 9; i++) add += parseInt(cpf.charAt(i)) * (10 - i);
    rev = 11 - (add % 11);
    if (rev == 10 || rev == 11) rev = 0;
    if (rev != parseInt(cpf.charAt(9)))
        return false; add = 0;
    for (i = 0; i < 10; i++)
        add += parseInt(cpf.charAt(i)) *
            (11 - i); rev = 11 - (add % 11);
    if (rev == 10 || rev == 11) rev = 0;
    if (rev != parseInt(cpf.charAt(10)))
        return false;
    return true;

} // Valida CNPJ function validaCNPJ(CNPJ) { CNPJ=CNPJ.replace(/[^\d]+/g,''); var a=new Array();
var b = new Number;
var c = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
for (i = 0; i < 12; i++) {
    a[i] = CNPJ.charAt(i); b += a[i] *
        c[i + 1];
}

if ((x = b % 11) < 2) {
    a[12] = 0
} else {
    a[12] = 11 - x
};

b = 0;
for (y = 0; y < 13; y++) { b += (a[y] * c[y]); }

if ((x = b % 11) < 2) {
    a[13] = 0;
} else {
    a[13] = 11 - x;
};

if ((CNPJ.charAt(12) != a[12]) || (CNPJ.charAt(13) != a[13])) {
    return false;
} if (CNPJ == "00000000000000") {
    return false;
} return true; 