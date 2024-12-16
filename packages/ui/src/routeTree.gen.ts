/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file was automatically generated by TanStack Router.
// You should NOT make any changes in this file as it will be overwritten.
// Additionally, you should also exclude this file from your linter and/or formatter to prevent it from being checked or modified.

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as IndexImport } from './routes/index'
import { Route as InspectIndexImport } from './routes/inspect/index'
import { Route as DeployIndexImport } from './routes/deploy/index'
import { Route as ConfigureIndexImport } from './routes/configure/index'

// Create/Update Routes

const IndexRoute = IndexImport.update({
  id: '/',
  path: '/',
  getParentRoute: () => rootRoute,
} as any)

const InspectIndexRoute = InspectIndexImport.update({
  id: '/inspect/',
  path: '/inspect/',
  getParentRoute: () => rootRoute,
} as any)

const DeployIndexRoute = DeployIndexImport.update({
  id: '/deploy/',
  path: '/deploy/',
  getParentRoute: () => rootRoute,
} as any)

const ConfigureIndexRoute = ConfigureIndexImport.update({
  id: '/configure/',
  path: '/configure/',
  getParentRoute: () => rootRoute,
} as any)

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/': {
      id: '/'
      path: '/'
      fullPath: '/'
      preLoaderRoute: typeof IndexImport
      parentRoute: typeof rootRoute
    }
    '/configure/': {
      id: '/configure/'
      path: '/configure'
      fullPath: '/configure'
      preLoaderRoute: typeof ConfigureIndexImport
      parentRoute: typeof rootRoute
    }
    '/deploy/': {
      id: '/deploy/'
      path: '/deploy'
      fullPath: '/deploy'
      preLoaderRoute: typeof DeployIndexImport
      parentRoute: typeof rootRoute
    }
    '/inspect/': {
      id: '/inspect/'
      path: '/inspect'
      fullPath: '/inspect'
      preLoaderRoute: typeof InspectIndexImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export interface FileRoutesByFullPath {
  '/': typeof IndexRoute
  '/configure': typeof ConfigureIndexRoute
  '/deploy': typeof DeployIndexRoute
  '/inspect': typeof InspectIndexRoute
}

export interface FileRoutesByTo {
  '/': typeof IndexRoute
  '/configure': typeof ConfigureIndexRoute
  '/deploy': typeof DeployIndexRoute
  '/inspect': typeof InspectIndexRoute
}

export interface FileRoutesById {
  __root__: typeof rootRoute
  '/': typeof IndexRoute
  '/configure/': typeof ConfigureIndexRoute
  '/deploy/': typeof DeployIndexRoute
  '/inspect/': typeof InspectIndexRoute
}

export interface FileRouteTypes {
  fileRoutesByFullPath: FileRoutesByFullPath
  fullPaths: '/' | '/configure' | '/deploy' | '/inspect'
  fileRoutesByTo: FileRoutesByTo
  to: '/' | '/configure' | '/deploy' | '/inspect'
  id: '__root__' | '/' | '/configure/' | '/deploy/' | '/inspect/'
  fileRoutesById: FileRoutesById
}

export interface RootRouteChildren {
  IndexRoute: typeof IndexRoute
  ConfigureIndexRoute: typeof ConfigureIndexRoute
  DeployIndexRoute: typeof DeployIndexRoute
  InspectIndexRoute: typeof InspectIndexRoute
}

const rootRouteChildren: RootRouteChildren = {
  IndexRoute: IndexRoute,
  ConfigureIndexRoute: ConfigureIndexRoute,
  DeployIndexRoute: DeployIndexRoute,
  InspectIndexRoute: InspectIndexRoute,
}

export const routeTree = rootRoute
  ._addFileChildren(rootRouteChildren)
  ._addFileTypes<FileRouteTypes>()

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/",
        "/configure/",
        "/deploy/",
        "/inspect/"
      ]
    },
    "/": {
      "filePath": "index.tsx"
    },
    "/configure/": {
      "filePath": "configure/index.tsx"
    },
    "/deploy/": {
      "filePath": "deploy/index.tsx"
    },
    "/inspect/": {
      "filePath": "inspect/index.tsx"
    }
  }
}
ROUTE_MANIFEST_END */
