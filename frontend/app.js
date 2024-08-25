const API_BASE_URL = process.env.REACT_APP_API_URL;

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
    return await response.json();
  } catch (error) {
    console.error('Request Error:', error);
    throw error;
  }
}

async function createStudyGroup(groupData) {
  return await sendRequest('study-groups', 'POST', groupData);
}

async function addParticipants(groupId, participants) {
  return await sendRequest(`study-groups/${groupId}/participants`, 'POST', { participants });
}

async function scheduleMeeting(groupId, meetingData) {
  return await sendRequest(`study-groups/${groupId}/meetings`, 'POST', meetingData);
}

async function displayStudyGroups() {
  const groups = await sendRequest('study-groups');
  const groupsContainer = document.getElementById('groups-list');
  groupsContainer.innerHTML = '';
  groups.forEach(group => {
    const groupElement = document.createElement('div');
    groupElement.textContent = `Group: ${group.name}`;
    groupsContainer.appendChild(groupElement);
  });
}

document.addEventListener('DOMContentLoaded', () => {
  document.getElementById('create-group-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const groupName = document.getElementById('group-name').value;
    await createStudyGroup({ name: groupName });
    displayStudyGroups();
  });
});

displayStudyGroups();