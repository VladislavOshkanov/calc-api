const selectorsContainer = document.getElementById("selectors");
const resultContainer = document.getElementById("result");
const formulaContainer = document.getElementById("formula_details");
const tariffBadge = document.getElementById("tariff_badge");
const statusPill = document.getElementById("status_pill");

const selectorOrder = [
    "place",
    "power",
    "kbm",
    "limitation",
    "age_experience",
    "season"
];

const selectorLabels = {
    place: "Территория регистрации",
    power: "Мощность двигателя",
    kbm: "КБМ",
    limitation: "Количество водителей",
    age_experience: "Возраст и стаж",
    season: "Период использования"
};

function compareRussianStrings(left, right) {
    return left.localeCompare(right, "ru", { sensitivity: "base" });
}

function formatNumber(value) {
    return new Intl.NumberFormat("ru-RU", {
        maximumFractionDigits: 2,
        minimumFractionDigits: 0
    }).format(value);
}

function formatMoney(value) {
    return `${formatNumber(value)} ₽`;
}

function renderOptionLabel(type, item) {
    if (type === "place") {
        return `${item.name} · КТ ${formatNumber(item.coefficent)}`;
    }

    if (type === "power") {
        const range = item.max_power >= 999
            ? `от ${item.min_power} л.с.`
            : `${item.min_power}-${item.max_power} л.с.`;
        return `${range} · КМ ${formatNumber(item.coefficent)}`;
    }

    if (type === "kbm") {
        return `Класс ${item.class} · КБМ ${formatNumber(item.coefficient)}`;
    }

    if (type === "limitation") {
        return item.limited
            ? `Ограниченный список · КО ${formatNumber(item.coefficient)}`
            : `Без ограничений · КО ${formatNumber(item.coefficient)}`;
    }

    if (type === "age_experience") {
        const fallbackLabel = `${item.age}+ лет, стаж ${item.experience}+ лет`;
        return `${item.label || fallbackLabel} · КВС ${formatNumber(item.coefficient)}`;
    }

    if (type === "season") {
        return `${item.months} мес. · КС ${formatNumber(item.coefficient)}`;
    }

    return item.name || JSON.stringify(item);
}

function createSelector(type, items) {
    const wrapper = document.createElement("label");
    wrapper.className = "selector-card";

    const title = document.createElement("span");
    title.className = "selector-card__title";
    title.textContent = selectorLabels[type];

    const select = document.createElement("select");
    select.id = type;

    const sortedItems = [...items];
    if (type === "place") {
        sortedItems.sort((left, right) => compareRussianStrings(left.name, right.name));
    }

    sortedItems.forEach((item) => {
        const option = document.createElement("option");
        option.value = item.id?.$oid || item._id?.$oid || item.id || item._id;
        option.textContent = renderOptionLabel(type, item);
        select.appendChild(option);
    });

    wrapper.append(title, select);
    return wrapper;
}

function pickLatestBasePrice(items) {
    return [...items].sort((left, right) => {
        const leftDate = left.created_at?.$date || left.created_at || "";
        const rightDate = right.created_at?.$date || right.created_at || "";
        return String(rightDate).localeCompare(String(leftDate));
    })[0];
}

async function loadModels() {
    selectorsContainer.innerHTML = '<p class="muted">Загружаем справочники…</p>';

    try {
        const response = await fetch("/api/all-models");
        const data = await response.json();

        selectorsContainer.innerHTML = "";
        selectorOrder.forEach((type) => {
            const items = data[type] || [];
            if (items.length > 0) {
                selectorsContainer.appendChild(createSelector(type, items));
            }
        });

        const latestBasePrice = pickLatestBasePrice(data.base_price || []);
        if (latestBasePrice) {
            tariffBadge.textContent = `Базовый тариф: ${formatMoney(latestBasePrice.min_base_price)}-${formatMoney(latestBasePrice.max_base_price)}`;
        }

        statusPill.hidden = false;
    } catch (error) {
        selectorsContainer.innerHTML = '<p class="muted">Не удалось загрузить справочники. Проверьте, что сервер и MongoDB запущены.</p>';
    }
}

function selectedPayload() {
    return {
        place_id: document.getElementById("place").value,
        power_id: document.getElementById("power").value,
        kbm_id: document.getElementById("kbm").value,
        limitation_id: document.getElementById("limitation").value,
        age_experience_id: document.getElementById("age_experience").value,
        season_id: document.getElementById("season").value
    };
}

function renderFormula(details) {
    formulaContainer.innerHTML = `
        <div class="formula-item"><span>КТ</span><strong>${formatNumber(details.place)}</strong></div>
        <div class="formula-item"><span>КМ</span><strong>${formatNumber(details.power)}</strong></div>
        <div class="formula-item"><span>КБМ</span><strong>${formatNumber(details.kbm)}</strong></div>
        <div class="formula-item"><span>КО</span><strong>${formatNumber(details.limitation)}</strong></div>
        <div class="formula-item"><span>КВС</span><strong>${formatNumber(details.age_experience)}</strong></div>
        <div class="formula-item"><span>КС</span><strong>${formatNumber(details.season)}</strong></div>
    `;
}

async function calculate() {
    resultContainer.innerHTML = '<p class="muted">Считаем стоимость…</p>';

    try {
        const response = await fetch("/api/calculate-coefficient", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(selectedPayload())
        });

        const data = await response.json();
        if (!response.ok) {
            resultContainer.innerHTML = `<p class="error-text">Ошибка: ${data.message || response.status}</p>`;
            return;
        }

        resultContainer.innerHTML = `
            <div class="result-grid">
                <div class="metric-card">
                    <span>Итоговый коэффициент</span>
                    <strong>${formatNumber(data.total_coefficient)}</strong>
                </div>
                <div class="metric-card">
                    <span>Минимальная цена</span>
                    <strong>${formatMoney(data.min_price)}</strong>
                </div>
                <div class="metric-card">
                    <span>Максимальная цена</span>
                    <strong>${formatMoney(data.max_price)}</strong>
                </div>
            </div>
        `;
        renderFormula(data.coefficients);
    } catch (error) {
        resultContainer.innerHTML = '<p class="error-text">Сервис недоступен. Проверьте соединение с сервером.</p>';
    }
}

document.getElementById("calculate").addEventListener("click", calculate);

loadModels();
