const messageBaseTime = 10;

function ajaxMessage(message, time = messageBaseTime) {
    const messageContainer = $('#message');
    const messageContent = $(message);
    const progressBar = messageContent.find('.j_progress_bar');

    messageContainer.empty();
    messageContainer.append(messageContent);

    progressBar.css('width', '0%');
    progressBar.animate({ width: '100%' }, time * 1000, 'linear', function () {
        messageContainer.fadeOut(300, function () {
            $(this).empty().show();
        });
    });

    messageContainer.effect('bounce', {times: 2, distance: 15 }, 300);
}

$('#message').on('click', '.j_message_close', function () {
    const messageContent = $(this).closest('.j_message');
    messageContent.stop(true);
    messageContent.fadeOut(300, function () {
        $(this).remove();
    });
});
