import { useEffect } from "react"
import { Routes, Route, useNavigate, useLocation } from "react-router-dom"
import { HomePage } from "@/pages/home"
// import { DashboardPage } from "@/pages/DashboardPage"
// import { HistoryPage } from "@/pages/HistoryPage"
import { SettingsPage } from "@/pages/settings"
import { SaveManagePage } from "@/pages/save-manage"
import { useSaves } from "@/contexts/saves"

export function App() {
  const navigate = useNavigate()
  const location = useLocation()
  const { saves, loaded } = useSaves()

  useEffect(() => {
    if (loaded && saves.length === 0 && location.pathname === "/") {
      navigate("/save-manage", { replace: true })
    }
  }, [loaded, saves, navigate, location.pathname])

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
