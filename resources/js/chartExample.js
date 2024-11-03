const ctx = document.getElementById('myChart');

new Chart(ctx, {
    type: 'bar',
    data: {
        labels: ['Janeiro', 'Fevereiro', 'Março', 'Abril', 'Maio', 'Junho', 'Julho', 'Agosto', 'Setembro', 'Outubro', 'Novembro'],
        datasets: [{
            label: 'Vendas',
            data: [2500, 7944, 5533, 2480, 6643, 1300, 2500, 1100, 1250, 1900, 5100],
            borderWidth: 1
        }]
    },
    options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
            y: {
                beginAtZero: true
            }
        }
    }
});

const ctx02 = document.getElementById('myChart02');

new Chart(ctx02, {
    type: 'bar',
    data: {
        labels: ['Setembro', 'Outubro', 'Novembro'],
        datasets: [{
            label: 'compras',
            data: [2543, 4575, 1592],
            backgroundColor: [
                'rgba(255, 99, 132, 0.2)',  // Cor da barra 1
                'rgba(54, 162, 235, 0.2)',  // Cor da barra 2
                'rgba(75, 192, 192, 0.2)',  // Cor da barra 3
            ],
            borderColor: [
                'rgba(255, 99, 132, 1)',  // Cor da borda da barra 1
                'rgba(54, 162, 235, 1)',  // Cor da borda da barra 2
                'rgba(75, 192, 192, 1)',  // Cor da borda da barra 3
            ],
            borderWidth: 1 // Espessura da borda
        }]
    },
    options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
            y: {
                beginAtZero: true
            }
        }
    }
});