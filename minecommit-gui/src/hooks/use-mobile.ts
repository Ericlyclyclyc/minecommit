import { useSyncExternalStore } from "react"

const MOBILE_BREAKPOINT = 768

function getMatchMedia() {
  return window.matchMedia(`(max-width: ${MOBILE_BREAKPOINT - 1}px)`)
}

function subscribe(callback: () => void) {
  const mql = getMatchMedia()
  mql.addEventListener("change", callback)
  return () => mql.removeEventListener("change", callback)
}

function getSnapshot() {
  return getMatchMedia().matches
}

export function useIsMobile() {
  const isMobile = useSyncExternalStore(subscribe, getSnapshot)
  return isMobile
}
