// Endpoints data
const endpoints = [
	{
		endpoint: '/health',
		method: 'GET',
		description: 'Health check endpoint',
		status: 'active',
	},
	{
		endpoint: '/api/v1/auth/register',
		method: 'POST',
		description: 'Register a new user account',
		status: 'active',
	},
	{
		endpoint: '/api/v1/auth/login',
		method: 'POST',
		description: 'Login with email and password',
		status: 'active',
	},
	,
	{
		endpoint: '/api/v1/auth/logout',
		method: 'POST',
		description: 'Logout the current user',
		status: 'active',
	},
	{
		endpoint: '/api/v1/auth/google',
		method: 'GET',
		description: 'Login with Google OAuth',
		status: 'active',
	},
];

// Function to get method badge class
function getMethodClass(method) {
	const methodLower = method.toLowerCase();
	const methodClasses = {
		get: 'get',
		post: 'post',
		put: 'put',
		delete: 'delete',
		patch: 'patch',
	};
	return methodClasses[methodLower] || 'get';
}

// Function to get status badge class
function getStatusClass(status) {
	const statusLower = status.toLowerCase();
	return statusLower === 'active' ? 'active' : 'inactive';
}

// Function to render table rows
function renderTable() {
	const tbody = document.querySelector('.endpoints-table tbody');

	if (!tbody) {
		console.error('Table tbody not found');
		return;
	}

	// Clear existing rows
	tbody.innerHTML = '';

	// Loop through endpoints and create rows
	endpoints.forEach((item) => {
		const row = document.createElement('tr');

		// Create cells
		const endpointCell = document.createElement('td');
		endpointCell.innerHTML = `<code>${item.endpoint}</code>`;

		const methodCell = document.createElement('td');
		const methodClass = getMethodClass(item.method);
		methodCell.innerHTML = `<span class="method-badge ${methodClass}">${item.method.toUpperCase()}</span>`;

		const descriptionCell = document.createElement('td');
		descriptionCell.textContent = item.description;

		const statusCell = document.createElement('td');
		const statusClass = getStatusClass(item.status);
		const statusText =
			item.status.charAt(0).toUpperCase() + item.status.slice(1);
		statusCell.innerHTML = `<span class="status-badge ${statusClass}">${statusText}</span>`;

		// Append cells to row
		row.appendChild(endpointCell);
		row.appendChild(methodCell);
		row.appendChild(descriptionCell);
		row.appendChild(statusCell);

		// Append row to tbody
		tbody.appendChild(row);
	});
}

// Render table when DOM is loaded
if (document.readyState === 'loading') {
	document.addEventListener('DOMContentLoaded', renderTable);
} else {
	// DOM is already loaded
	renderTable();
}
