const toastBaseTime = 5;

function ajaxToast(message, time = toastBaseTime) {
    const toast = $('#toast');
    const toastMessage = $(message).clone();
    const progressBar = toastMessage.find('.j_progress_bar');

    toast.append(toastMessage);

    progressBar.css('width', '0%');
    progressBar.animate({ width: '100%' }, time * 1000, 'linear', function () {
        toastMessage.fadeOut(300, function () {
            $(this).remove();
        });
    });

    toastMessage.effect('bounce', { times: 2, distance: 15 }, 300);
}

$('.j_toast').each(function (index, element) {
    setTimeout(() => ajaxToast(element), index * 1000);
});

$('#toast').on('click', '.j_toast_close', function () {
    const toast = $(this).closest('.j_toast');
    toast.stop(true);
    toast.fadeOut(300, function () {
        $(this).remove();
    });
});
