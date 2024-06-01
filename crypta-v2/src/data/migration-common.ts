export type MigrationQueryMap = {[userVersion: number]: string}
export type QueriesStringMap = {[path: string]: string}

export const defaultMigrationQueryMap: MigrationQueryMap = {
  0: '/db/0000_good_the_renegades.sql',
}
export const defaultQueriesStringMap: QueriesStringMap =
  import.meta.glob('/db/*.sql', {as: 'raw', eager: true})
