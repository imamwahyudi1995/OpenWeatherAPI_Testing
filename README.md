# Web Service Testing Samples


---

## 🔑 Global Variables (Profiles)

These variables are stored in the `Profiles > default` file:

| Variable     | Value               |
|--------------|---------------------|
| `apiKey`     | Your OpenWeatherMap API Key |
| `lat`        | `-6.2615` (Jakarta Selatan) |
| `lon`        | `106.8106` (Jakarta Selatan) |
| `units`      | `metric` (optional) |

---

## 🧪 Test Cases

### ✅ TC01_Get5DayForecast
- **Endpoint**: `/data/2.5/forecast`
- **Assertions**:
  - Status code = 200
  - Response has `city`, `list`
  - JSON schema validation
  - Temperature, weather description exist

### ✅ TC02_GetCurrentAirPollution
- **Endpoint**: `/data/2.5/air_pollution`
- **Assertions**:
  - Status code = 200
  - Response has `coord`, `list[0].components`
  - AQI value range (1-5)
  - All components are numbers

---

## ▶️ How to Run

1. Open the project in **Katalon Studio**
2. Go to **Test Suites > TS_WeatherAndAirPollution**
3. Click **Run** (choose desired environment)
4. Reports will be generated in `Reports/` directory

---

## 📊 How to Generate Report

1. Run a **Test Suite**
2. Go to **Reports**
3. Choose the latest report
4. Right-click > **Open with > Report**

---

## 💡 Notes

- Use your own **API key** and stay within free-tier rate limits.
- Variables are externalized to the Global Profile.
- Assertions include status code, JSON schema, and key attributes.

---

## 🌐 Reference

- [OpenWeatherMap API Docs](https://openweathermap.org/api)

---
