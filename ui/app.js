Vue.component('paginate', VuejsPaginate)

new Vue({
  el: '#app',
  data: {
    stats: {},
    sources: [],
    logs: [],
    currentPage: 1, // Current page number
    totalPages: 1,
    opened_source: null,
    searchFilter: null,
  },
  mounted() {
    // Fetch initial log data when the app is mounted
    this.fetchSources();
  },
  computed: {
    debouncedSearch() {
      return _.debounce(this.searchFilter, 3000); // Debounce the search function with a 3-second delay
    },
  },
  watch: {
    searchQuery() {
      this.debouncedSearch(); // Trigger the debounced search function whenever searchQuery changes
    },
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
      this.opened_source = sourceId;
      // Make a request to the API endpoint
      fetch(`/api/source/${sourceId}/logs?current_page=${this.currentPage}` + (this.searchFilter ? `&search=${this.searchFilter}` : `` ))
        .then(response => response.json())
        .then(data => {
          // Update the log data and metrics
          this.logs = data.items;
          this.totalPages = data.total_pages;
          this.currentPage = data.current_page;
        })
        .then(() => {
          this.$forceUpdate();
        })
        .catch(error => {
          console.error('Error fetching log data:', error);
        });
    },
    setCurrentPage(page) {
      this.currentPage = page;
      this.fetchLogsBySource(this.opened_source);
    },
  }
});
