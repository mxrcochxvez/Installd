import React from 'react';

export interface Release {
  id: string;
  version: string;
  name: string;
  description: string;
  date: string;
  downloadUrl: string;
  isPrerelease: boolean;
  size: string;
  downloads_count: number;
}

export interface Feature {
  title: string;
  description: string;
  icon: React.ReactNode;
}