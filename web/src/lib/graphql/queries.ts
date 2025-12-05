import { gql } from "@urql/core";

export const ME_QUERY = gql`
  query Me {
    me {
      id
      email
      name
      avatarUrl
    }
  }
`;

export const PROJECTS_QUERY = gql`
  query Projects {
    projects {
      id
      slug
      name
      description
      public
      createdAt
    }
  }
`;

export const PROJECT_QUERY = gql`
  query Project($slug: String!) {
    project(slug: $slug) {
      id
      slug
      name
      description
      public
      createdAt
      branches {
        id
        name
      }
      testbeds {
        id
        name
      }
      measures {
        id
        name
        units
      }
      benchmarks {
        id
        name
      }
      thresholds {
        id
        branchId
        testbedId
        measureId
        upperBoundary
        lowerBoundary
        minSampleSize
      }
      recentReports(limit: 10) {
        id
        gitHash
        createdAt
        branch {
          id
          name
        }
        testbed {
          id
          name
        }
      }
    }
  }
`;

export const PERF_QUERY = gql`
  query Perf(
    $projectSlug: String!
    $benchmarks: [ID!]!
    $branches: [ID!]!
    $measures: [ID!]!
    $testbeds: [ID!]!
    $startDate: DateTime
    $endDate: DateTime
  ) {
    perf(
      projectSlug: $projectSlug
      benchmarks: $benchmarks
      branches: $branches
      measures: $measures
      testbeds: $testbeds
      startDate: $startDate
      endDate: $endDate
    ) {
      series {
        benchmark {
          id
          name
        }
        branch {
          id
          name
        }
        testbed {
          id
          name
        }
        measure {
          id
          name
          units
        }
        data {
          x
          y
          lower
          upper
          gitHash
        }
      }
    }
  }
`;

export const ALERTS_QUERY = gql`
  query Alerts($projectSlug: String!, $status: AlertStatus) {
    alerts(projectSlug: $projectSlug, status: $status) {
      id
      baselineValue
      percentChange
      status
      createdAt
      metric {
        id
        value
        benchmark {
          id
          name
        }
        measure {
          id
          name
        }
      }
    }
  }
`;
