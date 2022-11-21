export function random_vector(vector_length) {
    return Array.from({ length: vector_length }, () => Math.random());
}

export function random_city() {
    return cities[Math.round(Math.random()*(cities.length-1))];
}

var cities = [
    "Tokyo",
    "Delhi",
    "Shanghai",
    "São Paulo",
    "Mexico City",
    "Cairo",
    "Mumbai",
    "Beijing",
    "Dhaka",
    "Osaka",
    "New York City",
    "Karachi",
    "Buenos Aires",
    "Chongqing",
    "Istanbul",
    "Kolkata",
    "Manila",
    "Lagos",
    "Rio de Janeiro",
    "Tianjin",
    "Kinshasa",
    "Guangzhou",
    "Los Angeles",
    "Moscow",
    "Shenzhen",
    "Lahore",
    "Bangalore",
    "Paris",
    "Bogotá",
    "Jakarta",
    "Chennai",
    "Lima",
    "Bangkok",
    "Seoul",
    "Nagoya",
    "Hyderabad",
    "London",
    "Tehran",
    "Chicago",
    "Chengdu",
    "Nanjing",
    "Wuhan",
    "Ho Chi Minh City",
    "Luanda",
    "Ahmedabad",
    "Kuala Lumpur",
    "Xi'an",
    "Hong Kong",
    "Dongguan",
    "Hangzhou"
]