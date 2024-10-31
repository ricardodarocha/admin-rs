/** EXIBIR/OCULTAR SENHA */
$(document).on('click', '.j_toggle_password', function () {
    let clicked = $(this);
    let input = $('#' + clicked.data('id'));

    if (clicked.hasClass('bi-eye')) {
        clicked.removeClass('bi-eye').addClass('bi-eye-slash');
        input.attr('type', 'text');
    } else {
        clicked.removeClass('bi-eye-slash').addClass('bi-eye');
        input.attr('type', 'password');
    }
});

/** EFEITO OUTLINE */
const outlineField = $('.j_outline_field');
const outlineContainer = $('.j_outline_container');

outlineField.each(fn => {
    if ($(this).is(':focus')) {
        $(this).closest(outlineContainer).removeClass('border-gray-300').addClass('border-gray-800');
    }
});

outlineField.on('focus', function () {
    $(this).closest(outlineContainer).removeClass('border-gray-300').addClass('border-gray-800');
});

outlineField.on('blur', function () {
    $(this).closest(outlineContainer).removeClass('border-gray-800').addClass('border-gray-300');
});