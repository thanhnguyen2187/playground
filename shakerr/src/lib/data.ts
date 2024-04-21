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

export function createNewItem(): Item {
  return {
    key: window.crypto.randomUUID().toString(),
    value: 'New',
    textColor: '#FFFFFF',
    backgroundColor: '#000000',
  }
}
