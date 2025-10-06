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