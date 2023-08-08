Vue.component('paginate', VuejsPaginate)
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
    opened_source: null,
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
    }
  },
  mounted() {
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
      this.fetchLogsBySource(sourceId);
    },
    fetchLogsBySource(sourceId) {
      this.opened_source = sourceId;
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
      if (this.opened_source == null){
        return null
      }
      // Make a request to the API endpoint
      fetch(`/api/source/${this.opened_source}/reset`)
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
        });
    },
    setCurrentPage(page) {
      this.currentPage = page;
      this.fetchLogsBySource(this.opened_source);
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
        return JSON.stringify(JSON.parse(message), null, 2);
      }catch(error) {
        return false
      }
    },
    debounceSearch(event) {
      clearTimeout(this.debounce)
      this.debounce = setTimeout(() => {
        this.searchFilter = event.target.value;
        this.currentPage = 1;
        this.fetchLogsBySource(this.opened_source);
      }, 600)
    },
  }
});
