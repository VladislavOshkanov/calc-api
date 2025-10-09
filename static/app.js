async function fetchModels(){
    const res = await fetch('/api/all-models');
    return res.json();
}

function createSelector(id,label,items){
    const wrapper = document.createElement('div');
    const lab = document.createElement('label'); lab.textContent = label; wrapper.appendChild(lab);
    const sel = document.createElement('select'); sel.id = id;
    items.forEach(it=>{
        const opt = document.createElement('option');
        opt.value = it.id || it._id || JSON.stringify(it);
        opt.textContent = it.name || it.title || opt.value;
        sel.appendChild(opt);
    });
    wrapper.appendChild(sel);
    return wrapper;
}

async function init(){
    const data = await fetchModels().catch(e=>{document.getElementById('selectors').textContent='Failed to load models';});
    if(!data) return;
    const container = document.getElementById('selectors');
    container.innerHTML='';
    container.appendChild(createSelector('age_experience','Age experience',data.age_experience));
    container.appendChild(createSelector('kbm','KBM',data.kbm));
    container.appendChild(createSelector('limitation','Limitation',data.limitation));
    container.appendChild(createSelector('place','Place',data.place));
    container.appendChild(createSelector('power','Power',data.power));
    container.appendChild(createSelector('season','Season',data.season));
}

async function calculate(){
    const payload = {
        age_experience_id: document.getElementById('age_experience').value,
        kbm_id: document.getElementById('kbm').value,
        limitation_id: document.getElementById('limitation').value,
        place_id: document.getElementById('place').value,
        power_id: document.getElementById('power').value,
        season_id: document.getElementById('season').value
    };
    const res = await fetch('/api/calculate-coefficient',{
        method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(payload)
    });
    const data = await res.json();
    document.getElementById('result').textContent = JSON.stringify(data);
}

document.getElementById('calculate').addEventListener('click',calculate);
init();

// --- Admin UI logic ---
function adminHeaders(){
    const token = document.getElementById('admin_token').value.trim();
    const headers = {'Content-Type':'application/json'};
    if(token) headers['Authorization'] = `Bearer ${token}`;
    return headers;
}

async function adminFetchList(){
    const type = document.getElementById('admin_model_type').value;
    const res = await fetch(`/admin/${type}`,{headers: adminHeaders()});
    if(!res.ok){ document.getElementById('admin_status').textContent = 'Failed to load: '+res.status; return; }
    const data = await res.json();
    const list = Array.isArray(data) ? data : (data.result || []);
    const container = document.getElementById('admin_list');
    container.innerHTML='';
    const ul = document.createElement('ul');
    list.forEach(it=>{
        const li = document.createElement('li');
        li.textContent = (it._id || it.id) + ' — ' + (it.name || it.title || JSON.stringify(it));
        ul.appendChild(li);
    });
    container.appendChild(ul);
}

async function adminCreate(){
    const type = document.getElementById('admin_model_type').value;
    let payload;
    try{ payload = JSON.parse(document.getElementById('admin_payload').value); }catch(e){ document.getElementById('admin_status').textContent='Invalid JSON'; return; }
    const res = await fetch(`/admin/${type}`,{method:'POST',headers:adminHeaders(),body:JSON.stringify(payload)});
    document.getElementById('admin_status').textContent = res.ok ? 'Created' : 'Failed: '+res.status;
    adminFetchList();
}

async function adminUpdate(){
    const type = document.getElementById('admin_model_type').value;
    const id = document.getElementById('admin_item_id').value.trim();
    if(!id){ document.getElementById('admin_status').textContent='Provide id'; return; }
    let payload;
    try{ payload = JSON.parse(document.getElementById('admin_payload').value); }catch(e){ document.getElementById('admin_status').textContent='Invalid JSON'; return; }
    const res = await fetch(`/admin/${type}/${id}`,{method:'PUT',headers:adminHeaders(),body:JSON.stringify(payload)});
    document.getElementById('admin_status').textContent = res.ok ? 'Updated' : 'Failed: '+res.status;
    adminFetchList();
}

async function adminDelete(){
    const type = document.getElementById('admin_model_type').value;
    const id = document.getElementById('admin_item_id').value.trim();
    if(!id){ document.getElementById('admin_status').textContent='Provide id'; return; }
    const res = await fetch(`/admin/${type}/${id}`,{method:'DELETE',headers:adminHeaders()});
    document.getElementById('admin_status').textContent = res.ok ? 'Deleted' : 'Failed: '+res.status;
    adminFetchList();
}

document.getElementById('admin_refresh').addEventListener('click',adminFetchList);
document.getElementById('admin_create').addEventListener('click',adminCreate);
document.getElementById('admin_update').addEventListener('click',adminUpdate);
document.getElementById('admin_delete').addEventListener('click',adminDelete);