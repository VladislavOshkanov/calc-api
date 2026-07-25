const adminTokenInput = document.getElementById("admin_token");
const adminModelSelect = document.getElementById("admin_model_type");
const adminPayload = document.getElementById("admin_payload");
const adminItemId = document.getElementById("admin_item_id");
const adminStatus = document.getElementById("admin_status");
const adminList = document.getElementById("admin_list");
const adminHint = document.getElementById("admin_hint");

const modelTemplates = {
    place: { name: "Москва", coefficient: 1.8 },
    power: { min_power: 101, max_power: 120, coefficient: 1.2 },
    kbm: { class: 3, coefficient: 1.17 },
    age_experience: {
        age: 22,
        experience: 3,
        coefficient: 1.13,
        label: "22-24 лет, стаж 3-4 года"
    },
    season: { months: 12, coefficient: 1.0 },
    limitation: { limited: true, coefficient: 1.0 },
    base_price: { min_base_price: 1399.0, max_base_price: 8665.0 }
};

function headers() {
    const token = adminTokenInput.value.trim();
    const result = { "Content-Type": "application/json" };
    if (token) {
        result.Authorization = `Bearer ${token}`;
    }
    return result;
}

function setStatus(message, isError = false) {
    adminStatus.textContent = message;
    adminStatus.dataset.state = isError ? "error" : "ok";
}

function prettyJson(value) {
    return JSON.stringify(value, null, 2);
}

function currentType() {
    return adminModelSelect.value;
}

function updateTemplate() {
    const type = currentType();
    adminPayload.value = prettyJson(modelTemplates[type]);
    adminHint.textContent = `Шаблон для ${type}. Поля можно редактировать перед отправкой.`;
}

function rememberToken() {
    localStorage.setItem("openapi_admin_token", adminTokenInput.value);
}

async function fetchList() {
    const type = currentType();
    setStatus(`Загружаем ${type}...`);

    try {
        const response = await fetch(`/admin/${type}`, { headers: headers() });
        if (!response.ok) {
            setStatus(`Не удалось загрузить список: ${response.status}`, true);
            return;
        }

        const data = await response.json();
        adminList.innerHTML = "";

        if (!Array.isArray(data) || data.length === 0) {
            adminList.innerHTML = '<p class="muted">Список пуст.</p>';
            setStatus(`Справочник ${type} пуст.`);
            return;
        }

        data.forEach((item) => {
            const card = document.createElement("button");
            card.type = "button";
            card.className = "record-card";
            card.innerHTML = `
                <span class="record-card__title">${item.name || item.label || `Запись ${item._id?.$oid || item.id?.$oid || ""}`}</span>
                <code>${item._id?.$oid || item.id?.$oid || ""}</code>
                <pre>${prettyJson(item)}</pre>
            `;
            card.addEventListener("click", () => {
                adminItemId.value = item._id?.$oid || item.id?.$oid || "";
                adminPayload.value = prettyJson(item);
                setStatus("Запись подставлена в форму.");
            });
            adminList.appendChild(card);
        });

        setStatus(`Загружено записей: ${data.length}`);
    } catch (error) {
        setStatus("API недоступен. Проверьте сервер и токен.", true);
    }
}

function parsePayload() {
    try {
        return JSON.parse(adminPayload.value);
    } catch (error) {
        setStatus("Некорректный JSON в payload.", true);
        return null;
    }
}

async function createItem() {
    const type = currentType();
    const payload = parsePayload();
    if (!payload) {
        return;
    }

    const response = await fetch(`/admin/${type}`, {
        method: "POST",
        headers: headers(),
        body: JSON.stringify(payload)
    });

    setStatus(response.ok ? "Запись создана." : `Ошибка создания: ${response.status}`, !response.ok);
    if (response.ok) {
        await fetchList();
    }
}

async function updateItem() {
    const type = currentType();
    const id = adminItemId.value.trim();
    if (!id) {
        setStatus("Укажите ID для обновления.", true);
        return;
    }

    const payload = parsePayload();
    if (!payload) {
        return;
    }

    const response = await fetch(`/admin/${type}/${id}`, {
        method: "PUT",
        headers: headers(),
        body: JSON.stringify(payload)
    });

    setStatus(response.ok ? "Запись обновлена." : `Ошибка обновления: ${response.status}`, !response.ok);
    if (response.ok) {
        await fetchList();
    }
}

async function deleteItem() {
    const type = currentType();
    const id = adminItemId.value.trim();
    if (!id) {
        setStatus("Укажите ID для удаления.", true);
        return;
    }

    const response = await fetch(`/admin/${type}/${id}`, {
        method: "DELETE",
        headers: headers()
    });

    setStatus(response.ok ? "Запись удалена." : `Ошибка удаления: ${response.status}`, !response.ok);
    if (response.ok) {
        adminItemId.value = "";
        await fetchList();
    }
}

adminTokenInput.value = localStorage.getItem("openapi_admin_token") || "";
adminTokenInput.addEventListener("input", rememberToken);
adminModelSelect.addEventListener("change", updateTemplate);

document.getElementById("admin_template").addEventListener("click", updateTemplate);
document.getElementById("admin_refresh").addEventListener("click", fetchList);
document.getElementById("admin_create").addEventListener("click", createItem);
document.getElementById("admin_update").addEventListener("click", updateItem);
document.getElementById("admin_delete").addEventListener("click", deleteItem);

updateTemplate();
if (adminTokenInput.value) {
    fetchList();
}
