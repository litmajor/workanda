export const CATEGORIES = [
  {
    id: 'writing',
    name: 'Writing & Translation',
    icon: '✍️',
    specializations: [
      { id: 'content-writing', name: 'Content Writing', niches: ['Blog Posts', 'SEO Articles', 'Website Content', 'Social Media'] },
      { id: 'copywriting', name: 'Copywriting', niches: ['Sales Copy', 'Email Marketing', 'Ad Copy', 'Product Descriptions'] },
      { id: 'technical-writing', name: 'Technical Writing', niches: ['API Documentation', 'User Manuals', 'White Papers', 'Technical Guides'] },
      { id: 'creative-writing', name: 'Creative Writing', niches: ['Fiction', 'Poetry', 'Scripts', 'Storytelling'] },
      { id: 'transcription', name: 'Transcription', niches: ['Legal Transcription', 'Medical Transcription', 'Law Enforcement', 'Academic', 'General'] },
      { id: 'translation', name: 'Translation', niches: ['Document Translation', 'Website Localization', 'Subtitle Translation', 'Legal Translation'] },
      { id: 'editing', name: 'Editing & Proofreading', niches: ['Copy Editing', 'Proofreading', 'Content Editing', 'Academic Editing'] },
    ],
  },
  {
    id: 'design',
    name: 'Design & Creative',
    icon: '🎨',
    specializations: [
      { id: 'graphic-design', name: 'Graphic Design', niches: ['Logo Design', 'Brand Identity', 'Print Design', 'Packaging'] },
      { id: 'ui-ux', name: 'UI/UX Design', niches: ['Web Design', 'Mobile App Design', 'User Research', 'Wireframing'] },
      { id: 'illustration', name: 'Illustration', niches: ['Digital Illustration', 'Character Design', 'Editorial', 'Children Books'] },
      { id: 'video-animation', name: 'Video & Animation', niches: ['2D Animation', '3D Animation', 'Motion Graphics', 'Video Editing'] },
      { id: 'photography', name: 'Photography', niches: ['Product Photography', 'Portrait', 'Event Photography', 'Real Estate'] },
    ],
  },
  {
    id: 'development',
    name: 'Development & IT',
    icon: '💻',
    specializations: [
      { id: 'web-development', name: 'Web Development', niches: ['Frontend (React, Vue, Angular)', 'Backend (Node.js, Python, Rust)', 'Full Stack', 'WordPress'] },
      { id: 'mobile-development', name: 'Mobile Development', niches: ['iOS (Swift)', 'Android (Kotlin)', 'React Native', 'Flutter'] },
      { id: 'software-development', name: 'Software Development', niches: ['Desktop Applications', 'Enterprise Software', 'SaaS', 'Custom Software'] },
      { id: 'game-development', name: 'Game Development', niches: ['Unity', 'Unreal Engine', 'Mobile Games', '2D Games'] },
      { id: 'blockchain', name: 'Blockchain & Crypto', niches: ['Smart Contracts', 'DeFi', 'NFT Projects', 'Web3'] },
      { id: 'devops', name: 'DevOps & Cloud', niches: ['AWS', 'Azure', 'Google Cloud', 'Docker/Kubernetes', 'CI/CD'] },
      { id: 'cybersecurity', name: 'Cybersecurity', niches: ['Penetration Testing', 'Security Audits', 'Network Security', 'Compliance'] },
    ],
  },
  {
    id: 'data-science',
    name: 'Data & Analytics',
    icon: '📊',
    specializations: [
      { id: 'data-science', name: 'Data Science', niches: ['Machine Learning', 'Deep Learning', 'AI Models', 'Predictive Analytics'] },
      { id: 'data-analysis', name: 'Data Analysis', niches: ['Business Intelligence', 'Data Visualization', 'Statistical Analysis', 'Excel/Tableau'] },
      { id: 'data-engineering', name: 'Data Engineering', niches: ['ETL Pipelines', 'Data Warehousing', 'Big Data', 'Database Design'] },
    ],
  },
  {
    id: 'marketing',
    name: 'Marketing & Sales',
    icon: '📈',
    specializations: [
      { id: 'digital-marketing', name: 'Digital Marketing', niches: ['SEO', 'SEM/PPC', 'Social Media Marketing', 'Email Marketing'] },
      { id: 'content-marketing', name: 'Content Marketing', niches: ['Content Strategy', 'Blogging', 'Video Marketing', 'Influencer Marketing'] },
      { id: 'sales', name: 'Sales', niches: ['Lead Generation', 'B2B Sales', 'Sales Copywriting', 'Cold Outreach'] },
      { id: 'market-research', name: 'Market Research', niches: ['Competitor Analysis', 'Consumer Research', 'Surveys', 'Market Reports'] },
    ],
  },
  {
    id: 'business',
    name: 'Business & Consulting',
    icon: '💼',
    specializations: [
      { id: 'business-consulting', name: 'Business Consulting', niches: ['Strategy', 'Operations', 'Management', 'Process Improvement'] },
      { id: 'financial-consulting', name: 'Financial Consulting', niches: ['Accounting', 'Bookkeeping', 'Financial Planning', 'Tax Consulting'] },
      { id: 'legal', name: 'Legal', niches: ['Contract Review', 'Legal Research', 'Compliance', 'IP Law'] },
      { id: 'hr', name: 'HR & Recruitment', niches: ['Recruiting', 'HR Consulting', 'Training & Development', 'Talent Management'] },
    ],
  },
  {
    id: 'admin',
    name: 'Admin & Customer Support',
    icon: '📝',
    specializations: [
      { id: 'virtual-assistant', name: 'Virtual Assistant', niches: ['General Admin', 'Email Management', 'Calendar Management', 'Data Entry'] },
      { id: 'customer-support', name: 'Customer Support', niches: ['Chat Support', 'Email Support', 'Phone Support', 'Technical Support'] },
      { id: 'project-management', name: 'Project Management', niches: ['Agile/Scrum', 'Project Coordination', 'Product Management', 'Program Management'] },
    ],
  },
  {
    id: 'engineering',
    name: 'Engineering & Architecture',
    icon: '⚙️',
    specializations: [
      { id: 'cad', name: 'CAD & 3D Modeling', niches: ['AutoCAD', '3D Modeling', 'Product Design', 'Architectural Rendering'] },
      { id: 'architecture', name: 'Architecture', niches: ['Residential', 'Commercial', 'Interior Design', 'Landscape'] },
      { id: 'mechanical', name: 'Mechanical Engineering', niches: ['Product Engineering', 'Manufacturing', 'Prototyping', 'Industrial Design'] },
    ],
  },
  {
    id: 'audio-music',
    name: 'Audio & Music',
    icon: '🎵',
    specializations: [
      { id: 'music-production', name: 'Music Production', niches: ['Beat Making', 'Mixing & Mastering', 'Sound Design', 'Composition'] },
      { id: 'voice-over', name: 'Voice Over', niches: ['Narration', 'Commercials', 'Audiobooks', 'Character Voices'] },
      { id: 'audio-editing', name: 'Audio Editing', niches: ['Podcast Editing', 'Audio Cleanup', 'Sound Effects', 'Audio Restoration'] },
    ],
  },
];

export const AFRICAN_REGIONS = [
  { id: 'west-africa', name: 'West Africa', countries: ['Nigeria', 'Ghana', 'Senegal', 'Ivory Coast', 'Mali'] },
  { id: 'east-africa', name: 'East Africa', countries: ['Kenya', 'Tanzania', 'Uganda', 'Ethiopia', 'Rwanda'] },
  { id: 'south-africa', name: 'Southern Africa', countries: ['South Africa', 'Botswana', 'Namibia', 'Zimbabwe', 'Zambia'] },
  { id: 'north-africa', name: 'North Africa', countries: ['Egypt', 'Morocco', 'Tunisia', 'Algeria', 'Libya'] },
  { id: 'central-africa', name: 'Central Africa', countries: ['Cameroon', 'DRC', 'Gabon', 'Chad', 'CAR'] },
];

export const PAYMENT_METHODS = [
  { id: 'mpesa', name: 'M-Pesa', regions: ['east-africa'], icon: '📱' },
  { id: 'flutterwave', name: 'Flutterwave', regions: ['west-africa', 'east-africa'], icon: '💳' },
  { id: 'paystack', name: 'Paystack', regions: ['west-africa'], icon: '💰' },
  { id: 'bank-transfer', name: 'Bank Transfer', regions: ['all'], icon: '🏦' },
  { id: 'crypto', name: 'Cryptocurrency', regions: ['all'], icon: '₿' },
  { id: 'stripe', name: 'Stripe', regions: ['all'], icon: '💵' },
];

export const CURRENCIES = [
  { code: 'USD', symbol: '$', name: 'US Dollar' },
  { code: 'EUR', symbol: '€', name: 'Euro' },
  { code: 'GBP', symbol: '£', name: 'British Pound' },
  { code: 'NGN', symbol: '₦', name: 'Nigerian Naira' },
  { code: 'KES', symbol: 'KSh', name: 'Kenyan Shilling' },
  { code: 'ZAR', symbol: 'R', name: 'South African Rand' },
  { code: 'GHS', symbol: 'GH₵', name: 'Ghanaian Cedi' },
  { code: 'EGP', symbol: 'E£', name: 'Egyptian Pound' },
];
