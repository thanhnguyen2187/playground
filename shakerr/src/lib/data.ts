export type Item = {
  key: string
  value: string
  textColor: string
  backgroundColor: string
}

export function loadItems() {
  return JSON.parse(window.localStorage.getItem('shakerr-items') ?? '[]') as Item[]
}

export function saveItems(items: Item[]) {
  window.localStorage.setItem('shakerr-items', JSON.stringify(items))
}

export function generateUUID() {
  const s4 = () => {
    return Math.floor((1 + Math.random()) * 0x10000)
    .toString(16)
    .substring(1);
  };
  return s4() + s4() + "-" + s4() + "-" + s4() + "-" + s4() + "-" + s4() + s4() + s4();
}

export function createNewItem(): Item {
  return {
    key: generateUUID(),
    value: 'New',
    textColor: '#FFFFFF',
    backgroundColor: '#000000',
  }
}
