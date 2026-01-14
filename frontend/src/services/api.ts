import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api';

const apiClient = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Content-Type': 'application/json',
    },
});

// Add auth token to requests
apiClient.interceptors.request.use((config) => {
    const token = localStorage.getItem('auth_token');
    if (token) {
        config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
});

export interface Member {
    id: string;
    email: string;
    name: string;
    farm_name?: string;
    location?: string;
    created_at: string;
}

export interface ProductListing {
    id: string;
    member_id: string;
    name: string;
    description: string;
    category: string;
    unit_price: string;
    quantity_available: number;
    unit_of_measure: string;
    availability_status: 'Available' | 'OutOfStock' | 'Archived';
    created_at: string;
    updated_at: string;
}

export interface Order {
    id: string;
    buyer_id: string;
    seller_id: string;
    listing_id: string;
    quantity: number;
    unit_price: string;
    total_amount: string;
    status: 'Pending' | 'Accepted' | 'Rejected' | 'Completed' | 'Cancelled';
    created_at: string;
    updated_at: string;
}

// Auth API
export const authAPI = {
    register: async (data: { email: string; password: string; name: string; farm_name?: string }) => {
        const response = await apiClient.post<{ member: Member; token: string }>('/auth/register', data);
        return response.data;
    },

    login: async (data: { email: string; password: string }) => {
        const response = await apiClient.post<{ member: Member; token: string }>('/auth/login', data);
        return response.data;
    },
};

// Listings API
export const listingsAPI = {
    getAll: async () => {
        const response = await apiClient.get<ProductListing[]>('/listings');
        return response.data;
    },

    getById: async (id: string) => {
        const response = await apiClient.get<ProductListing>(`/listings/${id}`);
        return response.data;
    },

    create: async (data: Partial<ProductListing>) => {
        const response = await apiClient.post<ProductListing>('/listings', data);
        return response.data;
    },

    update: async (id: string, data: Partial<ProductListing>) => {
        const response = await apiClient.put<ProductListing>(`/listings/${id}`, data);
        return response.data;
    },

    delete: async (id: string) => {
        await apiClient.delete(`/listings/${id}`);
    },
};

// Orders API
export const ordersAPI = {
    getAll: async () => {
        const response = await apiClient.get<Order[]>('/orders');
        return response.data;
    },

    getById: async (id: string) => {
        const response = await apiClient.get<Order>(`/orders/${id}`);
        return response.data;
    },

    create: async (data: { listing_id: string; quantity: number }) => {
        const response = await apiClient.post<Order>('/orders', data);
        return response.data;
    },

    updateStatus: async (id: string, status: Order['status']) => {
        const response = await apiClient.put<Order>(`/orders/${id}/status`, { status });
        return response.data;
    },
};

export default apiClient;
