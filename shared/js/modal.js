/** MODAL */
$('html, body').on('click', '[data-modal-open]', function (e) {
    e.preventDefault();
    e.stopPropagation();

    const clicked = $(this);
    const data = clicked.data();

    $.ajax({
        url: data.url,
        type: 'GET',
        data: data,
        dataType: 'JSON',
        success: function (response) {
            if (response.modal) {
                ajaxModal(response.modal.content);
            }
            if (response.message) {
                ajaxMessage(response.message);
            }
            if (response.toast) {
                ajaxToast(response.toast);
            }
        },
        error: function (jqXHR) {
            const errorResponse = jqXHR.responseJSON || {};
            if (errorResponse.message) {
                ajaxMessage(errorResponse.message);
            } else if (errorResponse.toast) {
                ajaxToast(errorResponse.toast);
            } else {
                const projectUrl = window.location.origin;
                $.get(projectUrl + '/shared/views/ajaxError.php', function (error) {
                    ajaxToast(error);
                });
            }
        }
    });
});

function ajaxModal(content) {
    let modal = $('#modal');
    let modalInputFocus = $('#modal-input-focus');

    if (modal.length) {
        modal.html(content);
        modal.fadeIn().css('display', 'flex');
        modalInputFocus.focus();
    } else {
        const projectUrl = window.location.origin;
        const message = encodeURIComponent('Este recurso está indisponível');
        $.get(projectUrl + '/shared/views/ajaxError.php?message=' + message, function (error) {
            ajaxToast(error);
        });
    }
}

$('html, body').on('click', '[data-modal-close]', function () {
    let modal = $('#modal');
    let modalContent = $('#modal-content');

    modal.fadeOut();
    modalContent.remove();
});
