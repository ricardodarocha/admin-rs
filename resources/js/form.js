/** FORMULARIO */

$('html, body').on('submit', 'form:not(.j_ajax_off)', function (e) {
    e.preventDefault();
    e.stopPropagation();

    const form = $(this);
    const method = form.find('input[name="_method"]').val() || form.attr('method');
    const load = form.find('.j_load');
    const btnSubmit = form.find('button[type=submit]');

    if(!method || !method.length){
        console.error('Método HTTP não especificado');
        return;
    }

    if (!form.ajaxSubmit) {
        console.error('jQuery Form Plugin não caregado.');
        return;
    }

    function toggleLoader(show) {
        if (show) {
            load.fadeIn().css('display', 'block');
            btnSubmit.css({'pointer-events': 'none', 'opacity': '0.7'});
        } else {
            load.fadeOut();
            btnSubmit.css({'pointer-events': 'auto', 'opacity': '1'});
        }
    }

    form.ajaxSubmit({
        url: form.attr('action'),
        type: method,
        dataType: 'json',
        beforeSend: function () {
            toggleLoader(true);
        },
        success: function (response) {
            if (response.redirect) {
                window.location.href = response.redirect;
            }
            if (response.reload) {
                window.location.reload();
            }
            if (response.message) {
                ajaxMessage(response.message);
            }
            if (response.toast) {
                alert(response.toast);
                ajaxToast(response.toast);
            }
        },
        complete: function () {
            toggleLoader(false);
        },
        error: function (jqXHR) {
            toggleLoader(false);

            const errorResponse = jqXHR.responseJSON || {};
            if (errorResponse.message) {
                ajaxMessage(errorResponse.message);
            } else if (errorResponse.toast) {
                ajaxToast(errorResponse.toast);
            } else {
                const projectUrl = window.location.origin;
                $.get(projectUrl + '/resources/views/components/ajaxError.html', function (error) {
                    ajaxToast(error);
                });
            }
        }
    });
});