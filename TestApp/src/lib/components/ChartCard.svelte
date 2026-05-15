<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    Chart,
    LineController,
    LineElement,
    PointElement,
    LinearScale,
    CategoryScale,
    Tooltip,
    Filler,
    Legend,
    type ChartConfiguration
  } from 'chart.js';

  Chart.register(
    LineController,
    LineElement,
    PointElement,
    LinearScale,
    CategoryScale,
    Tooltip,
    Filler,
    Legend
  );

  export let title: string = 'Graph';
  export let labels: (string | number)[] = [];
  export let values: number[] = [];
  export let color: string = '#2f855a';
  export let unit: string = '';

  let canvas: HTMLCanvasElement;
  let chart: Chart<'line'> | null = null;

  function buildConfig(): ChartConfiguration<'line'> {
    return {
      type: 'line',
      data: {
        labels,
        datasets: [
          {
            label: title,
            data: values,
            borderColor: color,
            backgroundColor: `${color}22`,
            borderWidth: 3.5,
            pointRadius: 0,
            pointHoverRadius: 5,
            tension: 0.2,
            fill: true
          }
        ]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        animation: false,
        interaction: {
          intersect: false,
          mode: 'index'
        },
        plugins: {
          legend: {
            display: false
          },
          tooltip: {
            backgroundColor: '#ffffff',
            titleColor: '#101820',
            bodyColor: '#101820',
            borderColor: '#101820',
            borderWidth: 2,
            displayColors: false,
            callbacks: {
              label: (context) => `${context.parsed.y}${unit}`
            }
          }
        },
        scales: {
          x: {
            grid: {
              color: 'rgba(16, 24, 32, 0.18)'
            },
            ticks: {
              color: '#101820',
              maxTicksLimit: 5,
              font: {
                size: 11,
                weight: 'bold'
              }
            },
            border: {
              color: '#101820',
              width: 2
            }
          },
          y: {
            grid: {
              color: 'rgba(16, 24, 32, 0.18)'
            },
            ticks: {
              color: '#101820',
              maxTicksLimit: 5,
              font: {
                size: 11,
                weight: 'bold'
              }
            },
            border: {
              color: '#101820',
              width: 2
            }
          }
        }
      }
    };
  }

  onMount(() => {
    chart = new Chart(canvas, buildConfig());
  });

  $: if (chart) {
    chart.data.labels = labels;
    chart.data.datasets[0].data = values;
    chart.update('none');
  }

  $: lastValue =
    values.length > 0 ? Number(values[values.length - 1]).toFixed(2) : '--';

  onDestroy(() => {
    if (chart) {
      chart.destroy();
      chart = null;
    }
  });
</script>

<article class="chart-card">
  <div class="chart-header">
    <div class="chart-title">{title}</div>
    <div class="chart-value">{lastValue}{unit}</div>
  </div>

  <div class="chart-canvas-wrap">
    <canvas bind:this={canvas}></canvas>
  </div>
</article>
