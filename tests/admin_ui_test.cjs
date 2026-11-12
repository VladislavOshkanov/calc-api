const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const vm = require('node:vm');

test('stored HTML is rendered as literal text in admin records', async () => {
    const nodes = new Map();
    function element() {
        return { value: '', children: [], dataset: {}, textContent: '',
            addEventListener() {}, append(...children) { this.children.push(...children); },
            appendChild(child) { this.children.push(child); },
            set innerHTML(value) {
                assert.ok(!value.includes('<img'), 'untrusted HTML must never reach innerHTML');
                this.children = [];
            }
        };
    }
    const attack = '<img src=x onerror="alert(document.cookie)">';
    const context = vm.createContext({
        document: {
            getElementById(id) { if (!nodes.has(id)) nodes.set(id, element()); return nodes.get(id); },
            createElement: element
        },
        localStorage: { getItem() { return ''; }, setItem() {} },
        fetch: async () => ({ ok: true, json: async () => [{name: attack, label: attack, _id: {$oid: '123'}}] })
    });
    vm.runInContext(readFileSync('static/admin.js', 'utf8'), context);
    nodes.get('admin_model_type').value = 'place';
    await vm.runInContext('fetchList()', context);
    const card = nodes.get('admin_list').children[0];
    assert.equal(card.children[0].textContent, attack);
    assert.equal(card.children[1].textContent, '123');
    assert.ok(card.children[2].textContent.includes('<img'));
});
