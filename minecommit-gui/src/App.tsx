import { Routes, Route } from "react-router-dom"
import { HomePage } from "@/pages/home"
// import { DashboardPage } from "@/pages/DashboardPage"
// import { HistoryPage } from "@/pages/HistoryPage"
import { SettingsPage } from "@/pages/settings"
import { SaveManagePage } from "@/pages/save-manage"

export function App() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      {/*<Route path="/dashboard" element={<DashboardPage />} />*/}
      {/*<Route path="/history" element={<HistoryPage />} />*/}
      <Route path="/settings" element={<SettingsPage />} />
      <Route path="/save-manage" element={<SaveManagePage />} />
    </Routes>
  )
}
