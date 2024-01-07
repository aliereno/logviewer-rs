Vue.component('paginate', VuejsPaginate)
Vue.component("vue-json-pretty", VueJsonPretty.default)
Vue.use(window['VueToastification'].default);

new Vue({
  el: '#app',
  data: {
    badge_classes: [
      "badge bg-info text-dark",
      // "badge bg-primary",
      "badge bg-secondary",
      "badge bg-success",
      // "badge bg-danger",
      "badge bg-warning text-dark",
      "badge bg-info text-dark",
      // "badge bg-light text-dark",
      "badge bg-dark"
    ],
    stats: {},
    sources: [],
    logs: [],
    currentPage: 1, // Current page number
    totalPages: 1,
    totalCount: 0,
    openedSource: null,
    searchFilter: null,
    selectedLog: null,
    selectedLogIndex: null,
    detailSpans: true,
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
  },
  mounted() {
    this.initializeCharts();

    // Fetch initial log data when the app is mounted
    this.fetchSources();

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
      this.selectedLogIndex = null;
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
          this.selectedLogIndex = null;
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
      this.selectedLogIndex = null;
      this.fetchLogsBySource(this.openedSource);
    },
    toggleDetails(index) {
      this.selectedLogIndex = index;
      this.selectedLog = this.logs[index];
      const offcanvasElement = new bootstrap.Offcanvas(document.getElementById('offcanvasScrolling'));

      // Toggle the Offcanvas
      offcanvasElement.toggle();
    },
    truncateMessage(message) {
      const firstLine = message.split('\n')[0];
      return firstLine.length > 250 ? `${firstLine.slice(0, 250)}...` : firstLine;
    },
    debounceSearch(event) {
      clearTimeout(this.debounce)
      this.debounce = setTimeout(() => {
        this.searchFilter = event.target.value;
        this.currentPage = 1;
        this.selectedLogIndex = null;
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
    },
    copyJsonValue(node) {
      let val = node.key + ': ' + node.content;
      navigator.clipboard.writeText(val);
      this.$toast.info("Copied to clipboard", {...this.toaster_context, timeout: 1800});
    },
    handleArrowKeys(event) {
      if (event.key === 'ArrowUp' || event.key === 'ArrowDown') {
        event.preventDefault(); // Prevent scrolling on arrow keys

        // Logic to update selected log index based on arrow key pressed
        if (event.key === 'ArrowUp' && this.selectedLogIndex > 0) {
          this.selectedLogIndex--;
        } else if (event.key === 'ArrowDown' && this.selectedLogIndex < this.logs.length - 1) {
          this.selectedLogIndex++;
        }
        
        // Perform any additional logic based on the selected log
        this.toggleDetails(this.selectedLogIndex);
      }
    },
    parseDetails(_json) {
      let _index = 0;
      let _len = this.badge_classes.length;
      let result = [];

      for (const [key, value] of Object.entries(_json)) {
        if (value.length < 100 && !(["", "{}"].includes(value))){
          result.push({
            "class": this.badge_classes[_index % _len],
            "value": `${key}: ${value}`
          });
          _index++;
        }
      }
      
      return result;
    },
    showDetailSpans() {
      this.detailSpans = !this.detailSpans
    }
  }
});
