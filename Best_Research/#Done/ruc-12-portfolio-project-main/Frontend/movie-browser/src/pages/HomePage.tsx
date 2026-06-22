// src/pages/HomePage.tsx
import styled from "styled-components";
import { UpNextModule, type UpNextItem } from "../features/Tim/Movies/UpNextModule";
import LegacyWidget from "../features/Tim/js-in-ts/LegacyWidget";

export const HomePage = () => {
  return (
    <Page>
      <MainModule>
        <MainImageWrapper>
          <HeroPanel>
            <Eyebrow>Now playing</Eyebrow>
            <HeroTitle>Deep-space stories, ready to explore.</HeroTitle>
            <HeroCopy>
              Discover new worlds, follow distant signals, and keep your next watchlist close.
            </HeroCopy>
          </HeroPanel>
        </MainImageWrapper>

        <UpNextWrapper>
          <UpNextModule items={upNextItems} />
        </UpNextWrapper>
      </MainModule>
      <LegacyWidget title={"Hello from JS! Using JSDOC for type checking! Find me at the HomePage.tsx ;)"} />
    </Page>
  );
};

const upNextItems: UpNextItem[] = [
  {
    id: "1",
    title: "The Expanse: New Horizons",
    description:
      "A rogue mining crew uncovers an ancient signal that threatens to destabilize the entire solar system.",
    href: "/movies/the-expanse-new-horizons",
  },
  {
    id: "2",
    title: "Signal in the Void",
    description:
      "A lone operator on a deep space relay station starts receiving messages from a future that shouldn't exist.",
    href: "/movies/signal-in-the-void",
  },
  {
    id: "3",
    title: "Orbital Factor",
    description:
      "Engineers on a failing ringworld race to patch its crumbling infrastructure before gravity fails.",
    href: "/movies/orbital-factor",
  },
];

const Page = styled.main`
  max-width: 1300px;
  margin: 0 auto;
  padding: 1rem 1.5rem 4rem;
  display: flex;
  flex-direction: column;
  gap: 2rem;
`;

/**
 * Main layout row: image (left) + sidebar (right) on desktop,
 * stacked with image on top on mobile.
 */
const MainModule = styled.section`
  display: flex;
  flex-direction: row;
  align-items: stretch;
  gap: 2rem;
  width: 100%;

  @media (max-width: 900px) {
    flex-direction: column; /* image first, UpNext below */
  }
`;

/**
 * Image container – lets the image grow/shrink nicely.
 */
const MainImageWrapper = styled.div`
  flex: 2;
  min-width: 0;
  max-width: 66.6%;
  display: flex;
  align-items: stretch;
`;

const HeroPanel = styled.section`
  min-height: 28rem;
  padding: 2.5rem;
  border-radius: 12px;
  color: #f8fafc;
  display: flex;
  flex-direction: column;
  justify-content: end;
  background:
    radial-gradient(circle at 80% 20%, rgba(96, 165, 250, 0.8), transparent 24%),
    radial-gradient(circle at 30% 30%, rgba(168, 85, 247, 0.48), transparent 32%),
    linear-gradient(135deg, #0f172a, #172554 58%, #020617);
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
`;

const Eyebrow = styled.p`
  margin: 0 0 0.75rem;
  color: #bfdbfe;
  font-size: 0.8rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
`;

const HeroTitle = styled.h1`
  max-width: 16ch;
  margin: 0;
  font-size: clamp(2rem, 5vw, 4rem);
  line-height: 1.05;
`;

const HeroCopy = styled.p`
  max-width: 48ch;
  margin: 1rem 0 0;
  color: #cbd5e1;
  line-height: 1.6;
`;

/**
 * Sidebar container – no hard min-width on mobile,
 * constrained width only on larger screens.
 */
const UpNextWrapper = styled.div`
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: stretch;


`;
