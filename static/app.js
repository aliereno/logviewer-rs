new Vue({
  el: '#app',
  data: {
    stats: {},
    sources: [],
    logs: [],
    currentPage: 1, // Current page number
    totalPages: 1,
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
    fetchLogsBySource(sourceId) {
      // Make a request to the API endpoint
      fetch(`/api/source/${sourceId}/logs?page=${this.currentPage}`)
        .then(response => response.json())
        .then(data => {
          // Update the log data and metrics
          this.logs = data.items;
          this.totalPages = data.total_pages;
          this.currentPage = data.current_page;
        })
        .then(this.fetchStatsBySource(sourceId))
        .catch(error => {
          console.error('Error fetching log data:', error);
        });
    },
    fetchStatsBySource(sourceId) {
      // Make a request to the API endpoint
      fetch(`/api/source/${sourceId}/stats`)
        .then(response => response.json())
        .then(data => {
          // Update the log data and metrics
          this.stats = data;
        })
        .catch(error => {
          console.error('Error fetching stats data:', error);
        });
    },
    setCurrentPage(page) {
      this.currentPage = page;
      this.fetchLogs(); // Fetch logs for the selected page
    },
  }
});
