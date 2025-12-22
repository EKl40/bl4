// BL4 Save Editor API - HTTP only (no Tauri)

import type {
  SaveInfo,
  CharacterInfo,
  SetCharacterRequest,
  InventoryItem,
  ItemDetail,
  BankInfo,
  DiscoverResponse,
  ApiResponse,
} from '../types';

async function httpPost<T>(endpoint: string, body?: unknown): Promise<T> {
  const res = await fetch(`/api${endpoint}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: body ? JSON.stringify(body) : undefined,
  });
  const json: ApiResponse<T> = await res.json();
  if (!json.success) throw new Error(json.error || 'Unknown error');
  return json.data as T;
}

async function httpGet<T>(endpoint: string): Promise<T> {
  const res = await fetch(`/api${endpoint}`);
  const json: ApiResponse<T> = await res.json();
  if (!json.success) throw new Error(json.error || 'Unknown error');
  return json.data as T;
}

export async function discoverSaves(): Promise<DiscoverResponse> {
  return httpGet<DiscoverResponse>('/discover');
}

export async function openSave(path: string, steamId: string): Promise<SaveInfo> {
  return httpPost<SaveInfo>('/save/open', { path, steam_id: steamId });
}

export async function saveChanges(): Promise<void> {
  return httpPost<void>('/save');
}

export async function getSaveInfo(): Promise<SaveInfo | null> {
  return httpGet<SaveInfo | null>('/save/info');
}

export async function getCharacter(): Promise<CharacterInfo> {
  return httpGet<CharacterInfo>('/character');
}

export async function setCharacter(request: SetCharacterRequest): Promise<void> {
  return httpPost<void>('/character', request);
}

export async function getInventory(): Promise<InventoryItem[]> {
  return httpGet<InventoryItem[]>('/inventory');
}

export async function getBank(): Promise<BankInfo> {
  return httpGet<BankInfo>('/bank');
}

export async function getItemDetail(serial: string): Promise<ItemDetail> {
  return httpPost<ItemDetail>('/item/detail', { serial });
}

export async function connectDb(path: string): Promise<void> {
  return httpPost<void>('/db/connect', { path });
}

export async function syncToBank(serials: string[]): Promise<number> {
  return httpPost<number>('/bank/sync', { serials });
}

// Re-export types for convenience
export type {
  SaveInfo,
  CharacterInfo,
  SetCharacterRequest,
  InventoryItem,
  ItemDetail,
  BankInfo,
  DiscoverResponse,
  DiscoveredProfile,
  DiscoveredCharacter,
  PartDetail,
} from '../types';
