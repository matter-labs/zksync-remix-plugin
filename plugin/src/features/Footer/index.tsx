import React from 'react'
import { useAtomValue } from 'jotai'
import ZKsyncLogo from '@/components/ZKsyncLogo'
import { solidityVersionAtom } from '@/atoms'
import './style.css'

export const Footer = () => {
  const solidityVersion = useAtomValue(solidityVersionAtom)

  return (
    <div className="version-wrapper">
      <div>
        <label className="version-left">
          <span>Using</span>
          <span>zksolc-{solidityVersion}</span>
        </label>
      </div>
      <div className="version-right">
        <label className="nethermind-powered">
          <span>Powered by: </span>
          <ZKsyncLogo size="xs" />
        </label>
      </div>
    </div>
  )
}
