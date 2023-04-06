import Dexie from 'dexie';

export interface Favorite {
  id?: number;
  routeId: number;
  quayId: number;
}

export class Db extends Dexie {
  favorites: Dexie.Table<Favorite, number>;

  constructor() {
    super('buss2');
    this.version(4).stores({
      favorites: '++id, [quayId+routeId]',
    });
  }
}

export default new Db();
