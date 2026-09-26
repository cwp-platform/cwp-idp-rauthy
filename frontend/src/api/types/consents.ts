export interface ConsentDocRequest {
    id: string;
    title: string;
    url: string;
    required: boolean;
    reconfirm: string;
    enabled: boolean;
}

export interface ConsentDoc {
    id: string;
    title: string;
    url: string;
    required: boolean;
    reconfirm: string;
    enabled: boolean;
    version: number;
    updated_ts: number;
    created_by: string;
}

export interface ConsentDocPublic {
    id: string;
    title: string;
    url: string;
    required: boolean;
    version: number;
}

export interface UserConsent {
    user_id: string;
    consent_id: string;
    version: number;
    accept_ts: number;
    url_snapshot: string;
    content_hash?: string | null;
    location: string;
    withdrawn_at?: number | null;
}

export interface ConsentPendingItem {
    id: string;
    title: string;
    url: string;
    required: boolean;
    version: number;
}

export interface ConsentAcceptItem {
    id: string;
    version: number;
}

export interface ConsentAcceptRequest {
    consents: ConsentAcceptItem[];
}
