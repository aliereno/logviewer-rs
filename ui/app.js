Vue.component('paginate', VuejsPaginate)
Vue.component("vue-json-pretty", VueJsonPretty.default)
Vue.use(window['VueToastification'].default);

new Vue({
  el: '#app',
  data: {
    stats: {},
    sources: [],
    logs: [],
    currentPage: 1, // Current page number
    totalPages: 1,
    totalCount: 0,
    openedSource: null,
    searchFilter: null,
    selectedLog: null,
    logInDetails: null,
    debounce: null,
    toaster_context: {
      position: 'top-right',
      timeout: 5000,
      closeOnClick: true,
      pauseOnHover: true,
      draggable: true,
      draggablePercent: 0.6,
      showCloseButtonOnHover: false,
      hideProgressBar: false,
      closeButton: 'button',
    },
    ramChart: null,
    queueChart: null,
  },
  mounted() {
    this.initializeCharts();

    // Fetch initial log data when the app is mounted
    this.fetchSources();

    // Fetch data every 2 seconds
    setInterval(() => {
      this.fetchMetrics();
    }, 2000); // 2000 milliseconds = 2 seconds
  },
  methods: {
    fetchSources() {
      fetch('/api/source')
        .then(response => response.json())
        .then(data => {
          // Update the log data and metrics
          this.sources = data;
        })
        .catch(error => {
          console.error('Error fetching log data:', error);
        });

    },
    openLogsBySource(sourceId) {
      this.currentPage = 1;
      this.fetchLogsBySource(sourceId);
    },
    fetchLogsBySource(sourceId) {
      this.openedSource = sourceId;
      // Make a request to the API endpoint
      fetch(`/api/source/${sourceId}/logs?page_size=100&current_page=${this.currentPage}` + (this.searchFilter ? `&search=${this.searchFilter}` : `` ))
        .then(response => response.json())
        .then(data => {
          // Update the log data and metrics
          this.logs = data.items;
          this.totalPages = data.total_pages;
          this.currentPage = data.current_page;
          this.totalCount = data.total_count;
        })
        .then(() => {
          this.$forceUpdate();
        })
        .catch(error => {
          console.error('Error fetching log data:', error);
        });
    },
    resetIndexes() {
      if (this.openedSource == null){
        return null
      }
      // Make a request to the API endpoint
      fetch(`/api/source/${this.openedSource}/reset`)
      .then(response => {
        if (!response.ok) {
          throw new Error(`HTTP error ${response.status}`);
        }
        return response.json();
      })
        .then(data => {
          this.$toast.success(data.message, this.toaster_context);
        })
        .catch(error => {
          this.$toast.error(error.message, this.toaster_context);
        }).then(data => {
          this.fetchLogsBySource(this.openedSource);
        });
    },
    setCurrentPage(page) {
      this.currentPage = page;
      this.fetchLogsBySource(this.openedSource);
    },
    toggleDetails(log) {
      if (this.selectedLog === log) {
        this.selectedLog = null;
      } else {
        this.selectedLog = log;
      }
    },
    truncateMessage(message) {
      const firstLine = message.split('\n')[0];
      return firstLine.length > 250 ? `${firstLine.slice(0, 250)}...` : firstLine;
    },
    isJSON(message) {
      let copy = message;
      try {
        while (copy.indexOf(" ") != -1) {
          let json = this.formatJSON(copy);
          if (json) {
            return json
          }
          // TODO: look with regex ?
          copy = copy.substr(copy.indexOf(" ") + 1);
        }
        return null;
      } catch (error) {
        alert(error);
        return null;
      }
    },
    formatJSON(message) {
      try{
        return JSON.parse(message);
      }catch(error) {
        return false
      }
    },
    debounceSearch(event) {
      clearTimeout(this.debounce)
      this.debounce = setTimeout(() => {
        this.searchFilter = event.target.value;
        this.currentPage = 1;
        this.fetchLogsBySource(this.openedSource);
      }, 600)
    },
    initializeCharts() {
      const options = {
        scales: {
          yAxes: [{ ticks: { beginAtZero: true }}],
          xAxes: [{
            type: 'time',
            time: {
              unitStepSize: 30,
              unit: 'second'
            },
            gridlines: { display: false }
          }]
        },
        tooltips: {	enabled: false },
        responsive: true,
        maintainAspectRatio: false,
        animation: false
      };

      this.ramChart = new Chart(document.getElementById('ramChart').getContext('2d'), {
        type: 'line',
        data: {
          labels: [],
          datasets: [{
            label: 'Ram Usage',
            data: [],
            backgroundColor: '#f87979',
            lineTension: 0.2,
            pointRadius: 0,
          }]
        },
        options
      });
      this.queueChart = new Chart(document.getElementById('queueChart').getContext('2d'), {
        type: 'line',
        data: {
          labels: [],
          datasets: [{
            label: 'Queue Count',
            data: [],
            backgroundColor: '#7AF5F9',
            lineTension: 0.2,
            pointRadius: 0,
          }]
        },
        options
      });
    },
    fetchMetrics() {
      try {
        fetch('/api/stat')
          .then(response => response.json())
          .then(data => {
            const { ram_usage, queue_count } = data;
    
            this.updateChart(this.ramChart, ram_usage);
            this.updateChart(this.queueChart, queue_count);
          })
      } catch (error) {
        console.error('Error fetching metrics:', error);
      }
    },
    updateChart(chart, value) {

      const timestamp = Date.now();
			if (chart.data.labels.length > 50) {
				chart.data.datasets.forEach(function (dataset) { dataset.data.shift(); });
				chart.data.labels.shift();
			}

      chart.data.datasets[0].data.push(value);
			chart.data.labels.push(timestamp);
      chart.update();
    },
    copyJsonValue(node) {
      let val = node.key + ': ' + node.content;
      navigator.clipboard.writeText(val);
      this.$toast.info("Copied to clipboard", {...this.toaster_context, timeout: 1800});
    },
  }
});
