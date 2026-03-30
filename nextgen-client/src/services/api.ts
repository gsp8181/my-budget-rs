import API_BASE from '../config';

export interface DashboardData {
    amount: string;
    remaining_week: string;
    end_of_week: string;
    full_weekend: string;
    monthly_debits: string;
    monthly_credits: string;
    net_saved_this_month: string;
    card_held_total: string;
    net_saved_avg: string;
    saved_this_year: string;
    today: Transaction[];
}

interface Transaction {
    id: number;
    oldId: number | null;
    category: string;
    name: string;
    day: number;
    amount: string;
    cardid: number | null;
    dbName: string;
}

export async function fetchDashboardData(): Promise<DashboardData> {
    const response = await fetch(`${API_BASE}/api`);
    if (!response.ok) {
        throw new Error('Failed to fetch dashboard data');
    }
    return response.json();
}
