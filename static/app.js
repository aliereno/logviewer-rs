function update(data) {
    // Clear the table body
    const tableBody = document.getElementById('table-body');
    tableBody.innerHTML = '';

    // Populate the table with the fetched data
    data._success_requests.forEach(item => {
    const row = document.createElement('tr');
    const url = document.createElement('td');
    const timestamp = document.createElement('td');
    const method = document.createElement('td');
    const duration = document.createElement('td');
    const response_status_code = document.createElement('td');

    response_status_code.textContent = item.response_status_code;
    duration.textContent = item.duration;
    url.textContent = item.request_url;
    method.textContent = item.request_method;
    timestamp.textContent = item.timestamp;

    row.appendChild(url);
    row.appendChild(timestamp);
    row.appendChild(method);
    row.appendChild(duration);
    row.appendChild(response_status_code);
    tableBody.appendChild(row);
    });

    setTimeout(fetchData, 10000)
}

function fetchData() {
    const levelFilter = document.getElementById('levelFilter');
    const levelValue = levelFilter.value;

    // Make a request to fetch data from the API
    fetch(`api/log/stats?filter=${levelValue}`)
      .then(response => response.json())
      .then(data => {
        update(data)
      })
      .catch(error => {
        console.error('Error fetching data:', error);
      });
      
  }

  fetchData();
  