import { groupBy } from 'lodash-es';
import Button from './Button';
import { formatUserData, createDebouncedFunction } from './utils';

// Demo data
const users = [
  { name: 'john doe', email: 'john@example.com', role: 'admin', age: 30 },
  { name: 'jane smith', email: 'jane@example.com', role: 'user', age: 25 },
  { name: 'bob wilson', email: 'bob@example.com', role: 'admin', age: 35 },
];

// Group users by role using lodash-es
const groupedUsers = groupBy(users, 'role');

console.log('Remote app loaded!');
console.log('Grouped users by role:', groupedUsers);

// Create a debounced search function
const debouncedSearch = createDebouncedFunction((query) => {
  console.log('Searching for:', query);
}, 500);

// Demo the remote app
const app = document.getElementById('app');
if (app) {
  app.innerHTML = `
    <div>
      <h2>Remote App - Lodash-ES Demo</h2>
      <div>
        <h3>Users grouped by role:</h3>
        <pre>${JSON.stringify(groupedUsers, null, 2)}</pre>
      </div>
      <div>
        <h3>Formatted user data:</h3>
        <pre>${JSON.stringify(users.map(formatUserData), null, 2)}</pre>
      </div>
      <input id="search" placeholder="Type to search (debounced)..." style="padding: 8px; margin: 10px 0; width: 300px;" />
    </div>
  `;

  // Add event listener for debounced search
  const searchInput = document.getElementById('search');
  if (searchInput) {
    searchInput.addEventListener('input', (e) => {
      debouncedSearch(e.target.value);
    });
  }
}