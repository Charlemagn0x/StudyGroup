const API_BASE_URL = process.env.REACT_APP_API_URL;

const groupDetailsCache = new Map();

async function sendRequest(endpoint, method = 'GET', data = null) {
  const url = `${API_BASE_URL}/${endpoint}`;
  const options = { method, headers: {} };
  if (data) {
    options.headers['Content-Type'] = 'application/json';
    options.body = JSON.stringify(data);
  }
  try {
    const response = await fetch(url, options);
    if (!response.ok) throw new Error('Something went wrong with the request');
    return response.json();
  } catch (error) {
    console.error('Request Error:', error);
    throw error;
  }
}

async function createStudyGroup(groupData) {
  return sendRequest('study-groups', 'POST', groupData);
}

async function addParticipants(groupId, participants) {
  return sendRequest(`study-groups/${groupId}/participants`, 'POST', { participants });
}

async function scheduleMeeting(groupId, meetingData) {
  return sendRequest(`study-groups/${groupId}/meetings`, 'POST', meetingData);
}

async function getGroupDetails(groupId) {
  if (groupDetailsCache.has(groupId)) {
    return groupDetailsCache.get(groupId);
  } else {
    const response = await sendRequest(`study-groups/${groupId}`);
    groupDetailsCache.set(groupId, response);
    return response;
  }
}

async function displayStudyGroups() {
  const groups = await sendRequest('study-groups');
  const groupsContainer = document.getElementById('groups-list');
  groupsContainer.innerHTML = '';
  groups.forEach(group => {
    const groupElement = document.createElement('div');
    groupElement.textContent = `Group: ${group.name} - Click for details`;
    groupElement.style.cursor = 'pointer';
    
    groupElement.addEventListener('click', async () => {
      const details = await getGroupDetails(group.id);
      alert(`Details for ${details.name}: \nParticipants: ${details.participants?.length || 0}`);
    });
    
    groupsContainer.appendChild(groupElement);
  });
}

document.addEventListener('DOMContentLoaded', () => {
  document.getElementById('create-group-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const groupName = document.getElementById('group-name').value;
    await createStudyGroup({ name: groupName });
    groupDetailsCache.clear();
    displayStudyGroups();
  });
});

displayStudyGroups();